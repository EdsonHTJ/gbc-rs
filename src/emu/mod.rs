
use std::sync::{Arc, Mutex};
use std::thread;
use crate::bus::{BusMutex};
use crate::cpu::{CPU};
use crate::gfx::color::Color;
use crate::gfx::Gfx;
use std::time::Duration;
use crate::cartridge::{Cartridge, CARTRIDGE_SINGLETON};
use crate::cpu::interrupts::IFlagsRegister;
use crate::dma::DMA;
use crate::io::IO;
use crate::lcd::LCD;
use crate::ppu::{PPU};
use crate::tick::TickManager;
use crate::timer::Timer;

const SCALE: u32 = 4;

const WIDTH: u32 = 240 * SCALE;

const HEIGHT: u32 = 160 * SCALE;

const DEBUG_H: u32 = 32 * 8 * SCALE;

const DEBUG_W: u32 = 16 * 8 * SCALE;

pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub tm: TickManager,
    pub bus: BusMutex,
    pub cpu: Arc<Mutex<CPU>>,
    pub ppu: Arc<Mutex<PPU>>,
    pub gfx: Box<dyn Gfx>,
    pub debug_gfx: Box<dyn Gfx>,
    pub die: bool,
}

#[derive(Clone)]
pub struct GlobalContext {
    pub timer: Arc<Mutex<Timer>>,
    pub ppu: Option<Arc<Mutex<PPU>>>,
    pub io: Option<Arc<Mutex<IO>>>,
    pub bus: Option<BusMutex>,
    pub dma: Option<Arc<Mutex<DMA>>>,
    pub tick_manager: Option<TickManager>,
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        let timer = Arc::new(Mutex::new(Timer::new()));
        let mut ctx = GlobalContext {
            timer: timer.clone(),
            io: None,
            bus: None,
            dma: None,
            tick_manager: None,
            ppu: None,
        };

        let dma = Arc::new(Mutex::new(DMA::new(ctx.clone())));
        ctx.dma = Some(dma.clone());

        let ppu = Arc::new(Mutex::new(PPU::new(ctx.clone())));
        ctx.ppu = Some(ppu.clone());

        let tick_manager = TickManager::new(timer.clone(), ctx.clone());
        ctx.tick_manager = Some(tick_manager.clone());

        let io = Arc::new(Mutex::new(IO::new(ctx.clone())));
        ctx.io = Some(io.clone());

        let bus = BusMutex::new(ctx.clone());
        ctx.bus = Some(bus.clone());

        ctx
    }
}

impl EMU {
    pub fn default() -> EMU {
        let gfx = Box::new(crate::gfx::sdl::SDL::new(WIDTH, HEIGHT, false).unwrap());
        let debug_gfx = Box::new(crate::gfx::sdl::SDL::new(DEBUG_W, DEBUG_H, true).unwrap());

        let ctx = GlobalContext::new();
        let cpu = Arc::new(Mutex::new(CPU::new(ctx.clone())));

        let emu = EMU {
            paused: false,
            running: false,
            die: false,
            tm: ctx.tick_manager.clone().unwrap(),
            bus: ctx.bus.unwrap(),
            cpu,
            ppu: ctx.ppu.unwrap(),
            gfx,
            debug_gfx,
        };


        emu
    }

    pub fn delay(&self, duration_ms: u64) -> () {
        ::std::thread::sleep(Duration::from_millis(duration_ms as u64));
    }

    pub fn load_game(&mut self, filename: String) {
        let content = std::fs::read(&filename).unwrap();
        let new_cartridge = Cartridge::new(content).unwrap();
        let mut current_cartridge = CARTRIDGE_SINGLETON.lock().unwrap();
        *current_cartridge = Some(new_cartridge);
    }

    pub fn stop(&mut self) {
        self.die = true;
        self.running = false;
        self.cpu.lock().unwrap().halted = true;
    }

    pub fn cpu_run(cpu: Arc<Mutex<CPU>>) {
        loop {
            let mut cpu = cpu.lock().unwrap();
            cpu.step_cpu().unwrap();
        }
    }

    fn init_window(&mut self) {
        self.gfx.init();
        self.gfx.clear(Color::new(0, 0, 0));
        self.gfx.present();
    }

    fn init_debug_window(&mut self) {
        self.debug_gfx.init();
        self.debug_gfx.clear(Color::new(0, 0, 0));
        self.debug_gfx.present();
    }

    fn update_window(&mut self) {
        self.gfx.present();
    }

    fn draw_chunk(gfx: &mut Box<dyn Gfx>, x: u32, y: u32, color: Color) {
        for i in 0..SCALE {
            for j in 0..SCALE {
                gfx.draw_pixel((x * SCALE + i) as i32, (y * SCALE + j) as i32, color).unwrap();
            }
        }
    }

    fn display_tile(ppu: Arc<Mutex<PPU>>, gfx: &mut Box<dyn Gfx>, addr: u16, tile_num: u16, x: u32, y: u32) {
        let mut tile_addr = addr + (tile_num * 16);
        for i in 0..8 {
            let byte1 = ppu.lock().unwrap().vram_read(tile_addr);
            tile_addr += 1;
            let byte2 = ppu.lock().unwrap().vram_read(tile_addr);
            tile_addr += 1;
            for j in 0..8 {
                let mut color = (byte1 >> (7 - j)) & 1;
                color |= ((byte2 >> (7 - j)) & 1) << 1;
                let color = match color {
                    0 => Color::new(255, 255, 255),
                    1 => Color::new(192, 192, 192),
                    2 => Color::new(96, 96, 96),
                    3 => Color::new(0, 0, 0),
                    _ => Color::new(0, 0, 0),
                };
                Self::draw_chunk(gfx, x + j, y + i, color)
            }
        }
    }

    fn update_debug_window(&mut self) {
        self.debug_gfx.present();
        self.debug_gfx.clear(Color::from_hex(0x111111));
        let addr = 0x0;
        let mut x_draw = 0;
        let mut tile_num = 0;
        for i in 0..24 {
            for x in 0..16 {
                EMU::display_tile(self.ppu.clone(),&mut self.debug_gfx, addr, tile_num, x_draw + (x * 8), i * 8);
                tile_num += 1;
            }
        }
    }

    pub fn run(&mut self) -> () {
        println!("Running the emulator");
        self.running = true;

        self.init_window();
        self.init_debug_window();
        self.tm.set_ticks(0).unwrap();

        let cpu_ref = self.cpu.clone();
        thread::spawn(move || {
            EMU::cpu_run(cpu_ref);
        });


        loop {
            if self.die {
                break;
            }
            self.ui_step();
            self.delay(1);
        }
    }

    fn ui_step(&mut self) {

        //canvas.clear();
        self.update_window();
        self.update_debug_window();
        let event_pump = self.gfx.get_user_events();
        for event in &event_pump {
            match event {
                crate::gfx::UserEvents::Quit => {
                    println!("Quitting the emulator");
                    self.stop();
                }
                crate::gfx::UserEvents::KeyPressed(key) => {
                    println!("Key pressed: {}", key);
                }
                _ => {}
            }
        }

        // The rest of the game loop goes here...

        self.gfx.present();
    }
}
