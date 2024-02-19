use crate::debug::log::{Logger, LoggerTrait};
use crate::debug::trace::Trace;
use crate::io::IO;

impl IO {
    pub(crate) fn on_serial_update(&mut self) {
        if self.serial_control == 0x81 {
            self.serial_message.push(self.serial_data as char);
            if self.serial_message.contains("Failed") {
                Trace::print_last_static(10000);
                panic!("Serial transfer failed");
            }
            Logger::log(format!("Serial message: {} \n", self.serial_message));
            self.serial_control = 0x00;
        }
    }
}