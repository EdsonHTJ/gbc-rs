use crate::tick::TICKER_SINGLETON;

mod bus;
mod cartridge;
mod cpu;
mod debug;
mod dma;
mod emu;
mod gfx;
mod instructions;
mod io;
mod lcd;
mod ppu;
mod ram;
mod tick;
mod timer;
mod util;

fn main() {
    let mut emu = emu::EMU::default();
    //let filename = "./games/dmg-acid2.gb".to_string();
    let filename = "./games/03-op sp,hl.gb".to_string();
    //let filename = ".games/04-op r,imm.gb".to_string();

    //let filename = "./games/01-special.gb".to_string();
    emu.load_game(filename);
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!(
        "EMU ticks: {}",
        TICKER_SINGLETON.lock().unwrap().get_ticks()
    );

    println!("Cpu Trace:");
    debug::trace::Trace::print_last_static(20);
}
