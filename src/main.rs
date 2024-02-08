mod emu;
mod gfx;
mod cartridge;

fn main() {
    let mut emu = emu::EMU::default();
    emu.run();
    println!("EMU is paused: {}", emu.paused);
    println!("EMU is running: {}", emu.running);
    println!("EMU ticks: {}", emu.ticks);
}
