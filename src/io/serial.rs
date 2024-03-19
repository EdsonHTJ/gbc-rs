use crate::debug::log::{Logger, LoggerTrait};
use crate::io::IO;

impl IO {
    pub(crate) fn on_serial_update(&mut self) {
        if self.serial_control == 0x81 {
            self.serial_message.push(self.serial_data as char);
            Logger::log(format!("Serial message: {} \n", self.serial_message));
            self.serial_control = 0x00;
        }
    }
}