


const BG_WINDOW_MASK: u8 = (1 << 7);
const Y_FLIP_MASK: u8 = (1 << 6);
const X_FLIP_MASK: u8 = (1 << 5);
const PALETTE_NUMBER_MASK: u8 = (1 << 4);
const TILE_VRAM_BANK_MASK: u8 = (1 << 3);
const CGB_PALLETE_NUMBER_MASK: u8 = 0x03;

#[derive(Clone)]
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
    oam_ram: Vec<OAM>,
    vram: [u8; 0x2000],
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            oam_ram: vec![OAM::default(); 40],
            vram: [0; 0x2000],
        }
    }

    pub fn ppu_tick(&mut self) {

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

