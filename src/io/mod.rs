


#[derive(Debug)]
#[allow(dead_code)]
pub enum IoError {
    InvalidAddress,
}

pub struct IO {}

impl IO {
    pub fn new() -> IO {
        IO {}
    }

    pub fn read(&self, _address: u16) -> Result<u8, IoError> {
        Ok(0)
    }

    pub fn write(&self, _address: u16, _data: u8) -> Result<(), IoError>{
        Ok(())
    }
}