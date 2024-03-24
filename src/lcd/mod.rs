use std::sync::{Arc, Mutex};
use crate::dma::DMA;
use crate::util;

pub enum LCDMode {
    HBlank,
    VBlank,
    OAM,
    PixelTransfer,
}

impl LCDMode {
    pub fn from_u8(value: u8) -> LCDMode {
        let value = value & 0b11;
        match value {
            0 => LCDMode::HBlank,
            1 => LCDMode::VBlank,
            2 => LCDMode::OAM,
            3 => LCDMode::PixelTransfer,
            _ => LCDMode::HBlank,
        }
    }
}

pub enum StatSrc {
    HBlank,
    VBlank,
    OAM,
    LYC,
}

impl StatSrc {
    pub fn from_u8(value: u8) -> StatSrc {
        match value {
            0b100 => StatSrc::HBlank,
            0b1000 => StatSrc::VBlank,
            0b10000 => StatSrc::OAM,
            0b100000 => StatSrc::LYC,
            _ => StatSrc::HBlank,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            StatSrc::HBlank => 0b100,
            StatSrc::VBlank => 0b1000,
            StatSrc::OAM => 0b10000,
            StatSrc::LYC => 0b100000,
        }
    }
}

#[repr(C)]
pub struct LcdRegisters {
    pub lcdc: u8,
    pub lcds: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub ly: u8,
    pub ly_compare: u8,
    pub dma: u8,
    pub bg_palette: u8,
    pub obj_palette: [u8; 2],
    pub wy: u8,
    pub wx: u8,
    pub bg_colors: [u32; 4],
    pub sp1_colors: [u32; 4],
    pub sp2_colors: [u32; 4],
}

pub struct LCD {
    pub register: LcdRegisters,
    pub dma: Arc<Mutex<DMA>>,
}

const colors_default: [u32; 4] = [
    0xFF_FF_FF_FF,
    0xFF_AA_AA_AA,
    0xFF_55_55_55,
    0xFF_00_00_00,
];

impl LCD {
    pub fn new(dma: Arc<Mutex<DMA>>) -> LCD {
        let reg = LcdRegisters {
            lcdc: 0x91,
            lcds: 0,
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            ly_compare: 0,
            dma: 0,
            bg_palette: 0xFC,
            obj_palette: [0xFF, 0xFF],
            wy: 0,
            wx: 0,
            bg_colors: colors_default.clone(),
            sp1_colors: colors_default.clone(),
            sp2_colors: colors_default.clone(),
        };

        LCD {
            register: reg,
            dma,
        }
    }

    pub fn lcdc_bgw_enabled(&self) -> bool {
        util::check_bit(self.register.lcdc, 0)
    }

    pub fn lcdc_obj_enabled(&self) -> bool {
        util::check_bit(self.register.lcdc, 1)
    }

    pub fn lcdc_obj_height(&self) -> u8 {
        match util::check_bit(self.register.lcdc, 2) {
            true => 16,
            false => 8,
        }
    }

    pub fn lcdc_bg_map_area(&self) -> u16 {
        match util::check_bit(self.register.lcdc, 3) {
            true => 0x9C00,
            false => 0x9800,
        }
    }

    pub fn lcdc_bgw_data_area(&self) -> u16 {
        match util::check_bit(self.register.lcdc, 4) {
            true => 0x8000,
            false => 0x8800,
        }
    }

    pub fn lcdc_win_enabled(&self) -> bool {
        util::check_bit(self.register.lcdc, 5)
    }

    pub fn lcdc_win_map_area(&self) -> u16 {
        match util::check_bit(self.register.lcdc, 6) {
            true => 0x9C00,
            false => 0x9800,
        }
    }

    pub fn lcdc_display_enabled(&self) -> bool {
        util::check_bit(self.register.lcdc, 7)
    }

    pub fn lcds_mode_flag(&self) -> LCDMode {
        LCDMode::from_u8(self.register.lcds & 0b11)
    }

    pub fn lcds_mode_set(&mut self, mode: LCDMode) {
        self.register.lcds = (self.register.lcds & !(0b11) | (mode as u8));
    }

    pub fn lcds_lyc_flag(&self) -> bool {
        util::check_bit(self.register.lcds, 2)
    }

    pub fn lcds_lyc_set(&mut self, value: bool) {
        self.register.lcds = util::modify_bit(self.register.lcds, 2, value);
    }

    pub fn lcds_stat_int(&mut self, interrupt: StatSrc) -> u8 {
        self.register.lcds & interrupt.to_u8()
    }

    pub fn lcd_read(&self, mut address: u16) -> u8 {
        unsafe {
            let lcd_buff = std::slice::from_raw_parts(&self.register as *const LcdRegisters as *const u8, std::mem::size_of::<LcdRegisters>());
            if address >= 0xFF40 {
                address -= 0xFF40;
            }

            if address >= 0x40 {
                address -= 0x40;
            }

            lcd_buff[address as usize]
        }
    }

    pub fn update_palette(&mut self, data: u8, pal: u8) {
        match pal {
            0 => {
                self.register.bg_colors[0] = colors_default[(data & 0b11) as usize];
                self.register.bg_colors[1] = colors_default[((data >> 2) & 0b11) as usize];
                self.register.bg_colors[2] = colors_default[((data >> 4) & 0b11) as usize];
                self.register.bg_colors[3] = colors_default[((data >> 6) & 0b11) as usize];
            }
            1 => {
                self.register.sp1_colors[0] = colors_default[(data & 0b11) as usize];
                self.register.sp1_colors[1] = colors_default[((data >> 2) & 0b11) as usize];
                self.register.sp1_colors[2] = colors_default[((data >> 4) & 0b11) as usize];
                self.register.sp1_colors[3] = colors_default[((data >> 6) & 0b11) as usize];
            }
            2 => {
                self.register.sp2_colors[0] = colors_default[(data & 0b11) as usize];
                self.register.sp2_colors[1] = colors_default[((data >> 2) & 0b11) as usize];
                self.register.sp2_colors[2] = colors_default[((data >> 4) & 0b11) as usize];
                self.register.sp2_colors[3] = colors_default[((data >> 6) & 0b11) as usize];
            }
            _ => {}
        }
    }

    pub fn lcd_write(&mut self, mut address: u16, data: u8) {
        unsafe {
            let mut lcd_buff = std::slice::from_raw_parts_mut(&self.register as *const LcdRegisters as *mut u8, std::mem::size_of::<LcdRegisters>());
            if address >= 0xFF40u16 {
                address -= 0xFF40;
            }

            if address >= 0x40 {
                address -= 0x40;
            }

            lcd_buff[address as usize] = data;
        }

        match address {
            0x06 => {
                self.dma.lock().unwrap().dma_start(data);
            }
            0x07 => {
                self.update_palette(data, 0);
            }
            0x08 => {
                self.update_palette(data & !0b11, 1);
            }
            0x09 => {
                self.update_palette(data & !0b11, 2);
            }
            _ => {}
        }
    }
}



