mod gui;
use macroquad::prelude::*;

#[macroquad::main("Camera")]
pub async fn main() {
    loop {
        gui::gui();

        next_frame().await
    }
}