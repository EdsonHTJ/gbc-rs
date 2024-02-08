use std::time::Duration;
use crate::gfx::Gfx;
use crate::gfx::color::Color;
mod errors;

pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    pub rom: Vec<u8>,
    pub gfx: Box<dyn Gfx>,
}

impl EMU {
    pub fn default() -> EMU {
        EMU {
            paused: false,
            running: false,
            ticks: 0,
            rom: vec![],
            gfx: Box::new(crate::gfx::sdl::SDL::new().unwrap()),
        }
    }

    pub fn delay(&self, duration_ms: u64) -> () {
        ::std::thread::sleep(Duration::from_millis(duration_ms as u64));
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn emu_loop(&mut self) {
        let mut i = self.ticks;
        i = (i + 1) % 255;
        //canvas.clear();
        self.gfx.draw_pixel(i as i32, i as i32, Color::new(255, 0, 0)).unwrap();
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

    pub fn run(&mut self) -> () {
        println!("Running the emulator");
        self.running = true;

        self.gfx.init();
        self.gfx.clear(Color::new(0, 0, 0));
        self.gfx.present();

        let mut i = 0;

        'running: loop {
            if self.running == false {
                break 'running;
            }

            if self.paused == true {
                self.delay(100);
                continue 'running;
            }

            self.emu_loop();

            self.ticks += 1;
        }
    }
}