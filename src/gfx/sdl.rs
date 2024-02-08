use crate::gfx::{Gfx, GfxError, UserEvents};
use crate::gfx::color::Color;

pub struct SDL {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl SDL {
    pub fn new() -> Result<SDL, GfxError> {
        let sdl_context = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(e) => return Err(GfxError::InitError(e.to_string())),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(e) => return Err(GfxError::InitError(e.to_string())),
        };

        let window_result = video_subsystem.window("gba-rs", 800, 600)
            .position_centered()
            .build();

        let window = match window_result {
            Ok(window) => window,
            Err(e) => return Err(GfxError::InitError(e.to_string())),
        };

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(GfxError::InitError(e.to_string())),
        };

        let event_pump = match sdl_context.event_pump() {
            Ok(event_pump) => event_pump,
            Err(e) => return Err(GfxError::InitError(e.to_string())),
        };

        Ok(SDL {
            canvas,
            event_pump,
        })
    }
}
impl Gfx for SDL {
    fn init(&self) -> () {
        println!("Initializing SDL");
    }

    fn present(&mut self) -> () {
        self.canvas.present();
    }

    fn clear(&mut self, color: Color) -> () {
        self.canvas.set_draw_color(Color::from_hex(0x0));
        self.canvas.clear();
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: crate::gfx::color::Color) -> Result<(), crate::gfx::GfxError> {
        self.canvas.set_draw_color(color);
        return match self.canvas.draw_point(sdl2::rect::Point::new(x, y)) {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::gfx::GfxError::DrawError(e.to_string())),
        }
    }

    fn get_user_events(&mut self) -> Vec<UserEvents> {
        self.event_pump.poll_iter().map(|event| {
            match event {
                sdl2::event::Event::Quit {..} => UserEvents::Quit,
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => UserEvents::KeyPressed(keycode.name().to_string()),
                _ => UserEvents::Unknown,
            }
        }).collect()
    }
}
