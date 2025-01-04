use crate::cpu::CPU;
use crate::debug::formatter;
use crate::instructions::Instruction;

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
        print!("CPU state:\n");
        print!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} PC: {:04X} SP: {:04X}\n",
                 cpu.registers.a, cpu.registers.f, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.pc, cpu.registers.sp);
        print!("Fetch data: {:04X}\n", cpu.fetch_data);
        print!("Mem dest: {:04X}\n", cpu.mem_dest);
        print!("Current opcode: {:02X}\n", cpu.current_opcode);
    }

    fn log_instruction(instruction: &Instruction) {
        println!("Instruction: {:?}\n", instruction);
    }

    fn log_cpu_state_with_instruction(cpu: &CPU) {
        let to_print = formatter::format_cpu_state(cpu);
        println!("{}", to_print);
    }

    fn put_serial_char(c: char) {
        print!("{}", c);
    }
}
