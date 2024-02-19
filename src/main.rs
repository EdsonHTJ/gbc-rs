mod bus;
mod cartridge;
mod cpu;
mod emu;
mod gfx;
mod instructions;
mod ram;
mod util;
mod tick;
mod io;
mod debug;

fn main() {
    let mut emu = emu::EMU::default();
    let filename = "./games/01-special.gb".to_string();
    emu.load_game(filename);
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!("EMU ticks: {}", emu.tm.get_ticks().unwrap());

    println!("Cpu Trace:");
    debug::trace::Trace::print_last_static(20);
}
