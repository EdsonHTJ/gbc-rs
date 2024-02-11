pub const ROM_HEADER_START: usize = 0x100;
const ROM_CHECKSUM_START: usize = 0x134;
const ROM_CHECKSUM_END: usize = 0x14C;

#[derive(Debug)]
pub enum CartridgeError {
    InvalidRomData,
    ReadFromInvalidAddress,
}

pub struct RomReader<'a> {
    pub start_offset: usize,
    pub rom_data: &'a Vec<u8>,
}

impl RomReader<'_> {
    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, CartridgeError> {
        if self.start_offset + size > self.rom_data.len() {
            return Err(CartridgeError::InvalidRomData);
        }

        let slice = &self.rom_data[self.start_offset..self.start_offset + size];
        self.start_offset += size;

        Ok(slice.to_vec())
    }
}

#[repr(C)]
pub struct RomHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: [u8; 16],
    pub new_lic_code: u16,
    pub sgb_flag: u8,
    pub cart_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub lic_code: u8,
    pub version: u8,
    pub checksum: u8,
    pub global_checksum: u16,
}

impl RomHeader {
    fn default() -> Self {
        RomHeader {
            entry: [0x00; 4],
            logo: [0x00; 0x30],
            title: [0x00; 16],
            new_lic_code: 0,
            sgb_flag: 0,
            cart_type: 0,
            rom_size: 0,
            ram_size: 0,
            dest_code: 0,
            lic_code: 0,
            version: 0,
            checksum: 0,
            global_checksum: 0,
        }
    }

    fn from_rom(rom_bytes: &Vec<u8>) -> Result<Self, CartridgeError> {
        let mut header = RomHeader::default();
        let mut reader = RomReader {
            start_offset: ROM_HEADER_START,
            rom_data: rom_bytes,
        };

        header.entry = reader.read_bytes(header.entry.len())?.try_into().unwrap();
        header.logo = reader.read_bytes(header.logo.len())?.try_into().unwrap();
        header.title = reader.read_bytes(header.title.len())?.try_into().unwrap();
        let lic_code = reader.read_bytes(2)?;
        header.new_lic_code = u16::from_be_bytes(lic_code.try_into().unwrap());
        header.sgb_flag = reader.read_bytes(1)?[0];
        header.cart_type = reader.read_bytes(1)?[0];
        header.rom_size = reader.read_bytes(1)?[0];
        header.ram_size = reader.read_bytes(1)?[0];
        header.dest_code = reader.read_bytes(1)?[0];
        header.lic_code = reader.read_bytes(1)?[0];
        header.version = reader.read_bytes(1)?[0];
        header.checksum = reader.read_bytes(1)?[0];
        let global_checksum = reader.read_bytes(2)?;
        header.global_checksum = u16::from_be_bytes(global_checksum.try_into().unwrap());

        Ok(header)
    }
}

pub struct Cartridge {
    pub rom_header: RomHeader,
    pub rom_data: Vec<u8>,
}

impl Cartridge {
    pub fn new(content: Vec<u8>) -> Result<Self, CartridgeError> {
        let rom_data = content;
        let rom_header = RomHeader::from_rom(&rom_data)?;

        Ok(Cartridge {
            rom_header,
            rom_data,
        })
    }

    pub fn validate_checksum(&self) -> bool {
        let mut sum: u16 = 0;
        for byte in &self.rom_data[ROM_CHECKSUM_START..ROM_CHECKSUM_END + 1] {
            sum = sum.wrapping_sub(*byte as u16).wrapping_sub(1);
        }

        (sum & 0xff) as u8 == self.rom_header.checksum
    }

    pub fn read_title(&self) -> String {
        let title = self.rom_header.title.to_vec();
        let title = title.split(|&x| x == 0).collect::<Vec<_>>()[0];
        String::from_utf8(title.to_vec()).unwrap()
    }

    pub fn read(&self, address: u16) -> Result<u8, CartridgeError> {
        // Using only rom for now
        if address as usize >= self.rom_data.len() {
            return Err(CartridgeError::ReadFromInvalidAddress);
        }

        Ok(self.rom_data[address as usize])
    }

    pub fn write(&mut self, _address: u16, _data: u8) -> Result<(), CartridgeError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartridge() {
        let filename = "./games/pokemon-y.gbc".to_string();
        let content = std::fs::read(&filename).unwrap();
        let cartridge = Cartridge::new(content).unwrap();
        assert_eq!(cartridge.read_title(), "POKEMON YELLOW".to_string());
    }

    #[test]
    fn test_validate_checksum() {
        let filename = "./games/pokemon-y.gbc".to_string();
        let content = std::fs::read(&filename).unwrap();
        let cartridge = Cartridge::new(content).unwrap();
        assert_eq!(cartridge.validate_checksum(), true);
    }
}
