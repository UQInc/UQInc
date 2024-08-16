use macroquad;
mod gui;
use macroquad::prelude::*;
struct Building {
    btype: String, // The type of building 
    width: i32,
    height: i32,
    sps: i32, // Students per Second that this building generates
    perkPoints: i32, // Number of perk points awarded by purchasing this building
}

struct Score {
    students: i32, // Total number of students cummulated
    currStudents: i32, // Current number of avaliable students
    totalSps: i32, // Total students per second value, what is being earnt
}

struct Event {
    studentsAwarded: i32, // Number of students this event gives (can be negative)
    eventType: String, // Type/Name of event
    duration: i32, // How long the even lasts (seconds)
    spsModifier: i32 // Multiplier that effects overall sps rate 
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
