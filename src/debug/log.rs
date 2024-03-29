use crate::cpu::CPU;
use crate::debug::formatter;
use crate::instructions::{Instruction};

pub trait LoggerTrait {
    fn log(message: String);
    fn log_cpu(cpu: &CPU);
    fn log_instruction(instruction: &Instruction);
    fn log_cpu_state_with_instruction(cpu: &CPU);

    fn put_serial_char(c: char);
}

pub struct Logger {}

#[cfg(not(feature = "log"))]
impl LoggerTrait for Logger {
    fn log(_message: String) {}

    fn log_cpu(_cpu: &CPU) {}

    fn log_instruction(_instruction: &Instruction) {}

    fn log_cpu_state_with_instruction(cpu: &CPU) {}

    fn put_serial_char(c: char) {}
}

#[cfg(feature = "log")]
impl LoggerTrait for Logger {
    fn log(message: String) {
        print!("{}", message);
    }

    fn log_cpu(cpu: &CPU) {
        println!("CPU state:");
        println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} PC: {:04X} SP: {:04X}",
                 cpu.registers.a, cpu.registers.f, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.pc, cpu.registers.sp);
        println!("Fetch data: {:04X}", cpu.fetch_data);
        println!("Mem dest: {:04X}", cpu.mem_dest);
        println!("Current opcode: {:02X}", cpu.current_opcode);
    }

    fn log_instruction(instruction: &Instruction) {
        println!("Instruction: {:?}", instruction);
    }

    fn log_cpu_state_with_instruction(cpu: &CPU) {
        let to_print = formatter::format_cpu_state(cpu);
        println!("{}", to_print);
    }



    fn put_serial_char(c: char) {
        print!("{}", c);
    }
}
