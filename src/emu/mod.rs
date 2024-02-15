
use std::sync::{Arc, Mutex};
use std::thread;
use crate::bus::{BusMutex};
use crate::cpu::{CPU};
use crate::gfx::color::Color;
use crate::gfx::Gfx;
use std::time::Duration;
use crate::tick::TickManager;


pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub tm: TickManager,
    pub bus: BusMutex,
    pub cpu: Arc<Mutex<CPU>>,
    pub gfx: Box<dyn Gfx>,
    pub die: bool,
}

impl EMU {
    pub fn default() -> EMU {
        let bus = BusMutex::new();
        let tm = TickManager::new();
        let emu = EMU {
            paused: false,
            running: false,
            die: false,
            tm: tm.clone(),
            bus: bus.clone(),
            cpu: Arc::new(Mutex::new(CPU::new(bus.clone(), tm.clone()))),
            gfx: Box::new(crate::gfx::sdl::SDL::new().unwrap()),
        };


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

    pub fn run(&mut self) -> () {
        println!("Running the emulator");
        self.running = true;

        self.gfx.init();
        self.gfx.clear(Color::new(0, 0, 0));
        self.gfx.present();
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
        let mut i = self.tm.get_ticks().unwrap();
        i = i + 1;
        //canvas.clear();
        self.gfx
          .draw_pixel(i as i32, i as i32, Color::new(255, 0, 0))
          .unwrap();
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
