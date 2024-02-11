mod bus;
mod cartridge;
mod cpu;
mod emu;
mod gfx;
mod instructions;
mod util;
mod ram;

fn main() {
    let mut emu = emu::EMU::default();
    let filename = "./games/mem_timing.gb".to_string();
    emu.load_game(filename);
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!("EMU ticks: {}", emu.ticks);
}
