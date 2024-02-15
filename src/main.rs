mod bus;
mod cartridge;
mod cpu;
mod emu;
mod gfx;
mod instructions;
mod log;
mod ram;
mod util;
mod tick;
mod io;

fn main() {
    let mut emu = emu::EMU::default();
    let filename = "./games/dmg-acid2.gb".to_string();
    emu.load_game(filename);
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!("EMU ticks: {}", emu.tm.get_ticks().unwrap());
}
