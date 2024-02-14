mod tick;

use std::sync::Arc;
use crate::bus::{BUS, BusMutex};
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::gfx::color::Color;
use crate::gfx::Gfx;
use crate::log::{Logger, LoggerTrait};
use std::time::Duration;
use crate::emu::tick::{TickManager};

pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub tm: TickManager,
    pub bus: BusMutex,
    pub cpu: CPU,
    pub gfx: Box<dyn Gfx>,
}

pub type FnCycle = Box<dyn FnMut(u32)>;

impl EMU {
    pub fn default() -> EMU {
        let bus = BusMutex::new();
        let mut emu = EMU {
            paused: false,
            running: false,
            tm: TickManager::new(),
            bus: bus.clone(),
            cpu: CPU::new(bus.clone()),
            gfx: Box::new(crate::gfx::sdl::SDL::new().unwrap()),
        };

        emu.cpu.set_fn_cycles(emu.get_cycles_callback());

        emu
    }

    pub fn delay(&self, duration_ms: u64) -> () {
        ::std::thread::sleep(Duration::from_millis(duration_ms as u64));
    }

    pub fn load_game(&mut self, filename: String) {
        let content = std::fs::read(&filename).unwrap();
        self.bus.load_game(content).unwrap();
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn emu_loop(&mut self) {
        let mut i = self.tm.get_ticks();
        i = (i + 1);
        //canvas.clear();
        //self.gfx
          //  .draw_pixel(i as i32, i as i32, Color::new(255, 0, 0))
          //  .unwrap();
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

        //self.gfx.present();
    }

    pub fn get_cycles_callback(&self) -> FnCycle {
        let mut tm = self.tm.clone();
        Box::new( move |c: u32| {
            tm.cycle()
        })
    }

    pub fn step_cpu(&mut self) -> () {
        if !self.cpu.halted {
            print!("Ticks: {:08X}", self.tm.get_ticks());
            let old_pc = self.cpu.registers.pc.clone();
            self.cpu.fetch_instruction().unwrap();
            print!(" OLDPC: {:04X} ", old_pc);
            Logger::log_cpu_state_with_instruction(&self.cpu);
            self.cpu.fetch_data().unwrap();
            self.cpu.execute().unwrap();
        } else {
            self.tm.cycle();
            if self.cpu.int_flags != 0 {
                self.cpu.halted = false;
            }
        }

        if self
            .cpu
            .get_interruption_master_enable()
            .unwrap()
            != 0
        {
            self.cpu.enable_ime = false;
        }

        if self.cpu.enable_ime {
            self.cpu
                .set_interruption_master_enable(1)
                .unwrap()
        }
    }

    pub fn run(&mut self) -> () {
        println!("Running the emulator");
        self.running = true;

        self.gfx.init();
        self.gfx.clear(Color::new(0, 0, 0));
        self.gfx.present();

        'running: loop {
            if self.running == false {
                break 'running;
            }

            if self.paused == true {
                self.delay(100);
                continue 'running;
            }

            self.step_cpu();
            self.emu_loop();

            self.tm.increment_ticks();
        }
    }
}
