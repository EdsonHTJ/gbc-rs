use crate::cpu::CPU;
use crate::instructions::{Instruction, RegType};

pub trait LoggerTrait {
    fn log(message: &str);
    fn log_cpu(cpu: &CPU);
    fn log_instruction(instruction: &Instruction);
    fn log_cpu_state_with_instruction(cpu: &CPU);
}

pub struct Logger {}


#[cfg(not(feature = "log"))]
impl LoggerTrait for Logger {
    fn log(_message: &str) {}

    fn log_cpu(_cpu: &CPU) {}

    fn log_instruction(_instruction: &Instruction) {}

    fn log_cpu_state_with_instruction(cpu: &CPU){}
}

#[cfg(feature = "log")]
impl LoggerTrait for Logger {
    fn log(message: &str) {
        println!("{}", message);
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
        //Print
        // PC: Instruction, AddressMode, Reg1, Reg2 A: B: C: D: E: H: L: F: SP:
        // Make it use the same space

        println!("PC: {:04X}: {:?} AddressMode: {:?} Reg1: {:?} Reg2: {:?} A: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} F: {:02X} SP: {:04X}",
                 cpu.registers.pc, cpu.current_instruction.type_, cpu.current_instruction.mode, cpu.current_instruction.reg_1, cpu.current_instruction.reg_2,
                 cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.f, cpu.registers.sp);
    }
}