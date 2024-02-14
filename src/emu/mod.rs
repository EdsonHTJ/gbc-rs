use std::sync::Arc;
use crate::bus::{BUS, BusMutex};
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::gfx::color::Color;
use crate::gfx::Gfx;
use crate::log::{Logger, LoggerTrait};
use std::time::Duration;

pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    pub bus: BusMutex,
    pub cpu: CPU,
    pub gfx: Box<dyn Gfx>,
}

pub type FnCycle<'a> = Box<dyn FnMut(u32) + 'a>;

impl EMU {
    pub fn default() -> EMU {
        let bus = BusMutex::new();
        EMU {
            paused: false,
            running: false,
            ticks: 0,
            bus: bus.clone(),
            cpu: CPU::new(bus.clone()),
            gfx: Box::new(crate::gfx::sdl::SDL::new().unwrap()),
        }
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
        let mut i = self.ticks;
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

    pub fn cycle(&mut self, cycles: u32) -> () {
        /*for _ in 0..cycles {
            for _ in 0..4 {
                self.ticks += 1;
            }
        }

        */
    }

    pub fn step_cpu(&mut self) -> () {
        if !self.cpu.halted {
            print!("Ticks: {:08X}", self.ticks);
            let old_pc = self.cpu.registers.pc.clone();
            self.cpu.fetch_instruction().unwrap();
            print!(" OLDPC: {:04X} ", old_pc);
            Logger::log_cpu_state_with_instruction(&self.cpu);
            let cycles = self.cpu.fetch_data().unwrap();
            self.cycle(cycles);
            self.cpu.execute().unwrap();
        } else {
            self.cycle(1);
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

            self.ticks += 1;
        }
    }
}
