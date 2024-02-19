use crate::cpu::CPU;

pub fn format_cpu_state(cpu: &CPU) -> String{
    return format!("TICKS: {:08X} PC: {:04X}: OP:{:02X} {:?} Data: {:04X} A: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} F: {:02X} SP: {:04X} Z: {} N: {} H: {} C: {}",
             cpu.tm.get_ticks().unwrap(),cpu.previous_pc, cpu.current_opcode,cpu.current_instruction.type_, cpu.fetch_data,
             cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.f, cpu.registers.sp, cpu.get_z_flag() as u8, cpu.get_n_flag() as u8, cpu.get_h_flag() as u8, cpu.get_c_flag() as u8);
}