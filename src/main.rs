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
    dps: i32, // Dollars per second value, what is being earnt
    dollars: i32, // Currency
}

struct Event {
    studentsAwarded: i32, // Number of students this event gives (can be negative)
    eventType: String, // Type/Name of event
    duration: i32, // How long the even lasts (seconds)
    spsModifier: i32 // Multiplier that effects overall sps rate 
}


impl Score {
    fn init() -> Self {
        Score {
            students: 0,
            currStudents: 0,
            dps: 1,
            dollars: 0,
        }
    }

    fn calc_dps(&self, modifiers: Vec<i32>) -> i32 {
        let mut new_sps: i32 = self.dps;
        for i in modifiers.iter() {
            new_sps *= i;
        }
        new_sps
    }

}

#[macroquad::main("Camera")]
pub async fn main() {
    loop {
        gui::gui();

        next_frame().await
    }
}

