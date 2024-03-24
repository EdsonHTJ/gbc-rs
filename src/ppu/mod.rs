use std::sync::{Arc, Mutex};
use crate::cpu::CPU;
use crate::cpu::interrupts::{IFlagsRegister, InterruptType};
use crate::emu::GlobalContext;
use crate::lcd::{LCD, LCDMode, StatSrc};
use crate::tick::TickManager;

const BG_WINDOW_MASK: u8 = 1 << 7;
const Y_FLIP_MASK: u8 = 1 << 6;
const X_FLIP_MASK: u8 = 1 << 5;
const PALETTE_NUMBER_MASK: u8 = 1 << 4;
const TILE_VRAM_BANK_MASK: u8 = 1 << 3;
const CGB_PALLETE_NUMBER_MASK: u8 = 0x03;

const LINES_PER_FRAME: u8 = 154;
const TICKS_PER_LINE: u32 = 456;
const YRES: u32 = 144;
const XRES: u32 = 160;

const TARGET_FRAME_TIME: u32 = 1000/60;

#[derive(Clone)]
#[derive(Copy)]
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

#[derive(Clone)]
pub struct PPU {
    oam_ram: [OAM; 40],
    vram: [u8; 0x2000],

    current_frame: u32,
    line_ticks: u32,
    video_buffer: [u32; (XRES * YRES) as usize],
    int_flags: Arc<Mutex<IFlagsRegister>>,
}

impl PPU {

    pub fn new(global_context: GlobalContext) -> PPU {
        PPU {
            oam_ram: [OAM::default(); 40],
            vram: [0; 0x2000],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: [0; (XRES * YRES) as usize],
            int_flags: global_context.int_flags.clone(),
        }
    }

    pub fn increment_ly(&mut self) {
        let mut lcd = LCD.lock().unwrap();

        if lcd.register.ly == lcd.register.ly_compare {
            lcd.lcds_lyc_set(true);
            if (lcd.lcds_stat_int(StatSrc::LYC) != 0) {
                self.int_flags.lock().unwrap().add_interrupt(InterruptType::LcdStat)
            }
        }else {
            lcd.lcds_lyc_set(false);
        }
    }

    pub fn ppu_mode_oam(&mut self) {
        if self.line_ticks >= 80 {
            LCD.lock().unwrap().lcds_mode_set(LCDMode::PixelTransfer);
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
                    self.int_flags.lock().unwrap().add_interrupt(InterruptType::VBlank);

                    if (lcd.lcds_stat_int(StatSrc::VBlank) != 0) {
                        self.int_flags.lock().unwrap().add_interrupt(InterruptType::LcdStat);
                    }

                    self.current_frame += 1;

                    //Calc fps

                   // let end = self.ppu_ticks.get_ticks().unwrap();
                }else {
                    lcd.lcds_mode_set(LCDMode::OAM);
                }
            }

            self.line_ticks = 0;
        }
    }

    pub fn ppu_mode_pixel_transfer(&mut self) {
        if self.line_ticks >= (0x80 + 172) {
            LCD.lock().unwrap().lcds_mode_set(LCDMode::HBlank);
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
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xFE00..=0xFE9F => {
                let oam_index = (address - 0xFE00) as usize / 4;
                match (address - 0xFE00) % 4 {
                    0 => self.oam_ram[oam_index].y,
                    1 => self.oam_ram[oam_index].x,
                    2 => self.oam_ram[oam_index].tile,
                    3 => self.oam_ram[oam_index].flags,
                    _ => 0,
                }
            },
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
            _ => {},
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

    pub fn vram_write(&mut self, address: u16, data: u8) {
        self.vram[(address) as usize] = data;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[(address) as usize]
    }

}

