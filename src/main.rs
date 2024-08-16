use macroquad;
mod gui;
use macroquad::prelude::*;

struct Building {
    btype: String,
    width: u32,
    height: u32,
}

#[macroquad::main("Camera")]
pub async fn main() {
    loop {
        gui::gui();

        next_frame().await
    }
}
