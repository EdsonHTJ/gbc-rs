use crate::tick::TICKER_SINGLETON;

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
mod timer;
mod ppu;
mod dma;
mod lcd;

fn main() {
    let mut emu = emu::EMU::default();
    let filename = "./games/02-interrupts.gb".to_string();
    emu.load_game(filename);
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!("EMU ticks: {}", TICKER_SINGLETON.lock().unwrap().get_ticks());

    println!("Cpu Trace:");
    debug::trace::Trace::print_last_static(20);
}
