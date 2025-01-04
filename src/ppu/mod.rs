use crate::bus::{self, BUS_SINGLETON};
use crate::cpu::interrupts::{IFlagsRegister, InterruptType, INTERRUPT_FLAGS};
use crate::cpu::CPU;
use crate::debug::log::{self, Logger, LoggerTrait};
use crate::lcd::{self, LCDMode, StatSrc, LCD};
use crate::tick::TickManager;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const BG_WINDOW_MASK: u8 = 1 << 7;
const Y_FLIP_MASK: u8 = 1 << 6;
const X_FLIP_MASK: u8 = 1 << 5;
const PALETTE_NUMBER_MASK: u8 = 1 << 4;
const TILE_VRAM_BANK_MASK: u8 = 1 << 3;
const CGB_PALLETE_NUMBER_MASK: u8 = 0x03;

const LINES_PER_FRAME: u8 = 154;
const TICKS_PER_LINE: u32 = 456;
pub const YRES: u8 = 144;
pub const XRES: u8 = 160;
pub const FULL_RES: u32 = (XRES as u32) * (YRES as u32);

const TARGET_FRAME_TIME: u32 = 1000 / 60;

#[derive(Clone, Copy)]
pub enum FetchState {
    FsTile,
    FsData0,
    FsData1,
    FsIdle,
    FsPush,
}

#[derive(Clone)]
pub struct Fifo {
    queue: Arc<Mutex<VecDeque<u32>>>,
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

#[derive(Clone)]
pub struct PixelFifoContext {
    pub current_fetch_state: FetchState,
    pub pixel_fifo: Fifo,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub bgw_fetch_data: [u8; 3],
    pub fetch_entry_data: [u8; 6],
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,
}

impl PixelFifoContext {
    pub fn new() -> PixelFifoContext {
        PixelFifoContext {
            current_fetch_state: FetchState::FsTile,
            pixel_fifo: Fifo::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct OAM {
    pub y: u8,
    pub x: u8,
    pub tile: u8,
    pub flags: u8,
}

impl OAM {
    pub fn default() -> OAM {
        OAM {
            y: 0,
            x: 0,
            tile: 0,
            flags: 0,
        }
    }

    pub fn get_bg_window_priority(&self) -> bool {
        (self.flags & BG_WINDOW_MASK) != 0
    }

    pub fn get_y_flip(&self) -> bool {
        (self.flags & Y_FLIP_MASK) != 0
    }

    pub fn get_x_flip(&self) -> bool {
        (self.flags & X_FLIP_MASK) != 0
    }

    pub fn get_palette_number(&self) -> bool {
        (self.flags & PALETTE_NUMBER_MASK) != 0
    }

    pub fn get_tile_vram_bank(&self) -> bool {
        (self.flags & TILE_VRAM_BANK_MASK) != 0
    }

    pub fn get_cgb_palette_number(&self) -> u8 {
        self.flags & CGB_PALLETE_NUMBER_MASK
    }
}

pub static PPU_SINGLETON: Lazy<Mutex<PPU>> = Lazy::new(|| Mutex::new(PPU::new()));
pub static VRAM: Lazy<Mutex<[u8; 0x2000]>> = Lazy::new(|| Mutex::new([0; 0x2000]));

#[derive(Clone)]
pub struct PPU {
    oam_ram: [OAM; 40],
    current_frame: u32,
    line_ticks: u32,
    video_buffer: [u32; FULL_RES as usize],
    previous_frame_time: u32,
    target_frame_time: u32,
    start_timer: u32,
    frame_count: u32,
    pixel_fifo_context: PixelFifoContext,
}

impl PPU {
    pub fn new() -> PPU {
        LCD.lock().unwrap().lcds_mode_set(LCDMode::OAM);
        PPU {
            oam_ram: [OAM::default(); 40],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: [0; FULL_RES as usize],
            previous_frame_time: 0,
            target_frame_time: TARGET_FRAME_TIME,
            start_timer: 0,
            frame_count: 0,
            pixel_fifo_context: PixelFifoContext::new(),
        }
    }

    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }

    pub fn get_video_buffer(&self) -> &[u32; FULL_RES as usize] {
        &self.video_buffer
    }

    pub fn increment_ly(&mut self) {
        let mut lcd = LCD.lock().unwrap();
        lcd.register.ly += 1;

        if lcd.register.ly == lcd.register.ly_compare {
            lcd.lcds_lyc_set(true);
            if lcd.lcds_stat_int(StatSrc::LYC) != 0 {
                INTERRUPT_FLAGS
                    .lock()
                    .unwrap()
                    .add_interrupt(InterruptType::LcdStat)
            }
        } else {
            lcd.lcds_lyc_set(false);
        }
    }

    pub fn ppu_mode_oam(&mut self) {
        if self.line_ticks >= 80 {
            LCD.lock().unwrap().lcds_mode_set(LCDMode::PixelTransfer);

            self.pixel_fifo_context.current_fetch_state = FetchState::FsTile;
            self.pixel_fifo_context.line_x = 0;
            self.pixel_fifo_context.pushed_x = 0;
            self.pixel_fifo_context.fetch_x = 0;
            self.pixel_fifo_context.fifo_x = 0;
        }
    }

    pub fn ppu_mode_vblank(&mut self) {
        if self.line_ticks >= (TICKS_PER_LINE) as u32 {
            self.increment_ly();
            {
                let mut lcd = LCD.lock().unwrap();
                if lcd.register.ly >= LINES_PER_FRAME {
                    lcd.lcds_mode_set(LCDMode::OAM);
                    lcd.register.ly = 0;
                }
            }

            self.line_ticks = 0;
        }
    }

    pub fn ppu_mode_hblank(&mut self) {
        if self.line_ticks >= (TICKS_PER_LINE) as u32 {
            self.increment_ly();
            {
                let mut lcd = LCD.lock().unwrap();
                if lcd.register.ly >= YRES as u8 {
                    lcd.lcds_mode_set(LCDMode::VBlank);
                    INTERRUPT_FLAGS
                        .lock()
                        .unwrap()
                        .add_interrupt(InterruptType::VBlank);

                    if lcd.lcds_stat_int(StatSrc::VBlank) != 0 {
                        INTERRUPT_FLAGS
                            .lock()
                            .unwrap()
                            .add_interrupt(InterruptType::LcdStat);
                    }

                    self.current_frame += 1;

                    //Calc fps

                    let end = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u32;
                    let frame_time = end - self.previous_frame_time;

                    if frame_time < self.target_frame_time {
                        let delay = self.target_frame_time - frame_time;
                        ::std::thread::sleep(Duration::from_millis(delay as u64));
                    }

                    if end - self.start_timer >= 1000 {
                        let fps = self.frame_count;
                        self.start_timer = end;
                        self.frame_count = 0;

                        Logger::log(format!("FPS: {}", fps));
                    }

                    self.frame_count += 1;
                    self.previous_frame_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u32;
                } else {
                    lcd.lcds_mode_set(LCDMode::OAM);
                }
            }

            self.line_ticks = 0;
        }
    }

    pub fn ppu_mode_pixel_transfer(&mut self) {
        self.pipeline_process();
        if self.pixel_fifo_context.pushed_x >= (XRES) {
            self.pipeline_fifo_reset();
            {
                let mut lcd = LCD.lock().unwrap();
                lcd.lcds_mode_set(LCDMode::HBlank);

                if lcd.lcds_stat_int(StatSrc::HBlank) != 0 {
                    INTERRUPT_FLAGS
                        .lock()
                        .unwrap()
                        .add_interrupt(InterruptType::LcdStat);
                }
            }
        }
    }

    pub fn ppu_tick(&mut self) {
        self.line_ticks += 1;
        let lcd_mode = LCD.lock().unwrap().lcds_mode_flag();
        match lcd_mode {
            LCDMode::HBlank => {
                self.ppu_mode_hblank();
            }
            LCDMode::VBlank => {
                self.ppu_mode_vblank();
            }
            LCDMode::OAM => {
                self.ppu_mode_oam();
            }
            LCDMode::PixelTransfer => {
                self.ppu_mode_pixel_transfer();
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => VRAM.lock().unwrap()[address as usize - 0x8000],
            0xFE00..=0xFE9F => {
                let oam_index = (address - 0xFE00) as usize / 4;
                match (address - 0xFE00) % 4 {
                    0 => self.oam_ram[oam_index].y,
                    1 => self.oam_ram[oam_index].x,
                    2 => self.oam_ram[oam_index].tile,
                    3 => self.oam_ram[oam_index].flags,
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    pub fn oam_write(&mut self, address: u16, data: u8) {
        let oam_index = (address) as usize / 4;
        match (address) % 4 {
            0 => self.oam_ram[oam_index].y = data,
            1 => self.oam_ram[oam_index].x = data,
            2 => self.oam_ram[oam_index].tile = data,
            3 => self.oam_ram[oam_index].flags = data,
            _ => {}
        }
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let oam_index = (address) as usize / 4;
        match (address) % 4 {
            0 => self.oam_ram[oam_index].y,
            1 => self.oam_ram[oam_index].x,
            2 => self.oam_ram[oam_index].tile,
            3 => self.oam_ram[oam_index].flags,
            _ => 0,
        }
    }

    pub fn vram_write(mut address: u16, data: u8) {
        if address >= 0x8000 {
            address -= 0x8000;
        }
        //log::Logger::log(format!("VRAM Write: {:#X} {:#X}\n", address + 0x8000, data));

        VRAM.lock().unwrap()[address as usize] = data;
    }

    pub fn vram_read(mut address: u16) -> u8 {
        if address >= 0x8000 {
            address -= 0x8000;
        }

        //log::Logger::log(format!("VRAM Read: {:#X}", address + 0x8000));

        VRAM.lock().unwrap()[address as usize]
    }

    fn pipeline_fifo_reset(&mut self) {
        self.pixel_fifo_context
            .pixel_fifo
            .queue
            .lock()
            .unwrap()
            .clear();
    }

    fn pixel_fifo_push(&mut self, pixel: u32) {
        self.pixel_fifo_context
            .pixel_fifo
            .queue
            .lock()
            .unwrap()
            .push_back(pixel);
    }

    fn pixel_fifo_pop(&mut self) -> u32 {
        self.pixel_fifo_context
            .pixel_fifo
            .queue
            .lock()
            .unwrap()
            .pop_front()
            .unwrap()
    }

    fn pixel_fifo_size(&self) -> usize {
        self.pixel_fifo_context
            .pixel_fifo
            .queue
            .lock()
            .unwrap()
            .len()
    }

    fn pipeline_fifo_add(&mut self) -> bool {
        if self.pixel_fifo_size() > 8 {
            return false;
        }

        let lcd = LCD.lock().unwrap();
        let x = self.pixel_fifo_context.fetch_x - (8 - lcd.register.scroll_x % 8);

        for i in 0..8 {
            let bit = 7 - i;
            let lo = (self.pixel_fifo_context.bgw_fetch_data[1] >> bit) & 1;
            let hi = (self.pixel_fifo_context.bgw_fetch_data[2] >> bit) & 1;
            let sum = (hi << 1) | lo;

            let color = lcd.register.bg_colors[sum as usize];

            if x >= 0 {
                self.pixel_fifo_push(color);
                self.pixel_fifo_context.fifo_x += 1;
            }
        }
        return true;
    }

    fn pipeline_fetch(&mut self) {
        match self.pixel_fifo_context.current_fetch_state {
            FetchState::FsTile => {
                let lcd = LCD.lock().unwrap();
                if lcd.lcdc_bgw_enabled() {
                    let mut bus = BUS_SINGLETON.lock().unwrap();
                    let addr = lcd.lcdc_bg_map_area()
                        + (self.pixel_fifo_context.map_x as u16) / 8
                        + ((self.pixel_fifo_context.map_y as u16) / 8) * 32;

                    self.pixel_fifo_context.bgw_fetch_data[0] = bus.read(addr).unwrap();
                    if lcd.lcdc_bgw_data_area() == 0x8800 {
                        self.pixel_fifo_context.bgw_fetch_data[0] =
                            self.pixel_fifo_context.bgw_fetch_data[0].wrapping_add(128);
                    }
                }

                self.pixel_fifo_context.current_fetch_state = FetchState::FsData0;
                self.pixel_fifo_context.fetch_x = self.pixel_fifo_context.fetch_x.wrapping_add(8);
            }
            FetchState::FsData0 => {
                let mut bus = BUS_SINGLETON.lock().unwrap();
                let lcd = LCD.lock().unwrap();

                let addr = lcd.lcdc_bgw_data_area()
                    + (self.pixel_fifo_context.bgw_fetch_data[0] as u16) * 16
                    + self.pixel_fifo_context.tile_y as u16;

                self.pixel_fifo_context.bgw_fetch_data[1] = bus.read(addr).unwrap();
                self.pixel_fifo_context.current_fetch_state = FetchState::FsData1;
            }
            FetchState::FsData1 => {
                let mut bus = BUS_SINGLETON.lock().unwrap();
                let lcd = LCD.lock().unwrap();

                let addr = lcd.lcdc_bgw_data_area()
                    + (self.pixel_fifo_context.bgw_fetch_data[0] as u16) * 16
                    + (self.pixel_fifo_context.tile_y + 1) as u16;

                self.pixel_fifo_context.bgw_fetch_data[2] = bus.read(addr).unwrap();
                self.pixel_fifo_context.current_fetch_state = FetchState::FsIdle;
            }
            FetchState::FsIdle => self.pixel_fifo_context.current_fetch_state = FetchState::FsPush,
            FetchState::FsPush => {
                if self.pipeline_fifo_add() {
                    self.pixel_fifo_context.current_fetch_state = FetchState::FsTile;
                }
            }
        }
    }

    fn pipeline_push_pixel(&mut self) {
        if self.pixel_fifo_size() > 8 {
            let pixel_data = self.pixel_fifo_pop();

            {
                let lcd = LCD.lock().unwrap();
                if self.pixel_fifo_context.line_x >= lcd.register.scroll_x % 8 {
                    let buffer_pos = self.pixel_fifo_context.pushed_x as usize
                        + (lcd.register.ly as usize * XRES as usize);
                    self.video_buffer[buffer_pos] = pixel_data;

                    self.pixel_fifo_context.pushed_x += 1;
                }

                self.pixel_fifo_context.line_x += 1;
            }
        }
    }

    fn pipeline_process(&mut self) {
        {
            let lcd = LCD.lock().unwrap();
            self.pixel_fifo_context.map_y = lcd.register.ly.wrapping_add(lcd.register.scroll_y);
            self.pixel_fifo_context.map_x = self
                .pixel_fifo_context
                .fetch_x
                .wrapping_add(lcd.register.scroll_x);
            self.pixel_fifo_context.tile_y = (self.pixel_fifo_context.map_y % 8) * 2;
        }

        if self.line_ticks & 1 == 0 {
            self.pipeline_fetch();
        }

        self.pipeline_push_pixel();
    }
}
