use macroquad;
mod gui;
use macroquad::prelude::*;
struct Building {
    btype: String,
    width: u32,
    height: u32,
}

mod music;

#[macroquad::main("Camera")]
pub async fn main() {
    let _music_handle = std::thread::spawn(|| {
        music::music();
    });

    loop {
        gui::gui();

        next_frame().await
    }
    
}
