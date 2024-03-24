use crate::gfx::color::Color;

pub(crate) mod color;
pub(crate) mod sdl;

#[derive(Debug)]
pub enum UserEvents {
    Unknown,
    Quit,
    KeyPressed(String),
}

#[derive(Debug)]
pub enum GfxError {
    InitError(String),
    DrawError(String),
}

pub trait Gfx {
    fn new(width: u32, height: u32, is_debug: bool) -> Result<Self, GfxError>
    where
        Self: Sized;
    fn init(&self) -> ();
    fn present(&mut self) -> ();
    fn clear(&mut self, color: Color) -> ();
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), GfxError>;
    fn get_user_events(&mut self) -> Vec<UserEvents>;
}

#[cfg(feature = "sdl")]
pub fn get_gfx(width: u32, height: u32, is_debug: bool) -> Result<Box<dyn Gfx>, GfxError> {
    sdl::SDL::new(width, height, is_debug).map(|sdl| Box::new(sdl) as Box<dyn Gfx>)
}

#[cfg(feature = "sdl")]
pub fn get_ticks() -> u32 {
    sdl::SDL::get_ticks()
}

pub fn delay(ms: u32) {
    #[cfg(feature = "sdl")]
    sdl::SDL::delay(ms)
}