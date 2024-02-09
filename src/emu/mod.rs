use std::time::Duration;
use crate::cartridge::Cartridge;
use crate::gfx::Gfx;
use crate::gfx::color::Color;

pub struct EMU {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    pub cartridge: Option<Cartridge>,
    pub gfx: Box<dyn Gfx>,
}

impl EMU {
    pub fn default() -> EMU {
        EMU {
            paused: false,
            running: false,
            ticks: 0,
            cartridge: None,
            gfx: Box::new(crate::gfx::sdl::SDL::new().unwrap()),
        }
    }

    pub fn delay(&self, duration_ms: u64) -> () {
        ::std::thread::sleep(Duration::from_millis(duration_ms as u64));
    }

    pub fn load_game(&mut self, filename: String) {
        let content = std::fs::read(&filename).unwrap();
        let cartridge = Cartridge::new(content).unwrap();
        self.cartridge = Some(cartridge);
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