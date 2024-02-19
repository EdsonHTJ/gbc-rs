use std::sync::Mutex;
use crate::debug::log::{Logger, LoggerTrait};

static TRACE: Mutex<Trace> = Mutex::new(Trace{trace: Vec::new()});

pub struct Trace {
    trace: Vec<String>
}

#[allow(dead_code)]
impl Trace {
    pub fn log(&mut self, message: String) {
        self.trace.push(message);
    }

    pub fn print(&self) {
        for line in &self.trace {
            Logger::log(line.to_string());
        }
    }

    pub fn clear(&mut self) {
        self.trace.clear();
    }

    pub fn print_last(&self, n: usize) {
        // Print the last n lines
        let start = if self.trace.len() > n {
            self.trace.len() - n
        } else {
            0
        };

        for i in start..self.trace.len() {
            Logger::log(self.trace[i].to_string() + "\n");
        }
    }

    pub fn print_first(&self, n: usize) {
        // Print the first n lines
        let end = if self.trace.len() > n {
            n
        } else {
            self.trace.len()
        };

        for i in 0..end {
            Logger::log(self.trace[i].to_string() + "\n");
        }
    }

    pub fn print_range(&self, end: usize, offset : usize) {
        // Print the first n lines
        let start = if self.trace.len() > end {
            end
        } else {
            self.trace.len()
        };

        for i in start-offset..start {
            Logger::log(self.trace[i].to_string() + "\n");
        }
    }
    //Static trace functions
    pub fn log_static(message: String) {
        let mut trace = TRACE.lock().unwrap();
        trace.log(message);
    }

    pub fn print_static() {
        let trace = TRACE.lock().unwrap();
        trace.print();
    }

    pub fn clear_static() {
        let mut trace = TRACE.lock().unwrap();
        trace.clear();
    }

    pub fn print_last_static(n: usize) {
        let trace = TRACE.lock().unwrap();
        trace.print_range(183980, 1000);
    }
}