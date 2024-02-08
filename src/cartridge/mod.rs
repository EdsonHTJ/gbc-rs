
pub struct RomReader<'a> {
    pub start_offset: usize,
    pub rom_data: &'a Vec<u8>,
}

impl RomReader<'_> {
    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let slice = &self.rom_data[self.start_offset..self.start_offset + size];
        self.start_offset += size;

        slice.to_vec()
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

    fn from_rom(rom_bytes: &Vec<u8>) -> Self {
        let mut header = RomHeader::default();
        let mut reader = RomReader{start_offset: 0, rom_data: rom_bytes};

        header.entry = reader.read_bytes(header.entry.len()).try_into().unwrap();
        header.logo = reader.read_bytes(header.logo.len()).try_into().unwrap();
        header.title = reader.read_bytes(header.title.len()).try_into().unwrap();
        let lic_code = reader.read_bytes(1);
        header.new_lic_code = u16::from_be_bytes(lic_code.try_into().unwrap());
        header.sgb_flag = reader.read_bytes(1)[0];
        header.cart_type = reader.read_bytes(1)[0];
        header.rom_size = reader.read_bytes(1)[0];
        header.ram_size = reader.read_bytes(1)[0];
        header.dest_code = reader.read_bytes(1)[0];
        header.lic_code = reader.read_bytes(1)[0];
        header.version = reader.read_bytes(1)[0];
        header.checksum = reader.read_bytes(1)[0];
        let global_checksum = reader.read_bytes(2);
        header.global_checksum = u16::from_be_bytes(global_checksum.try_into().unwrap());

        header
    }
}

pub struct Cartridge {
    pub filename: String,
    pub rom_header: RomHeader,
    pub rom_data: Vec<u8>,
}

impl Cartridge {
    pub fn new(filename: String, content: Vec<u8>) {
        let rom_data = content;

    }
}