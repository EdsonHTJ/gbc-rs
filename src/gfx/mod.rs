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
    fn init(&self) -> ();
    fn present(&mut self) -> ();
    fn clear(&mut self, color: Color) -> ();
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), GfxError>;
    fn get_user_events(&mut self) -> Vec<UserEvents>;
}
