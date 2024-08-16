use macroquad;
mod gui;
use macroquad::prelude::*;

struct Building {
    btype: String, // The type of building 
    width: i32,
    height: i32,
    sps: i32, // Students per Second that this building generates
    perk_points: i32, // Number of perk points awarded by purchasing this building
}

struct Score {
    students: i32, // Total number of students cummulated
    curr_students: i32, // Current number of avaliable students
    total_sps: i32, // Total students per second value, what is being earnt
}

struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String, // Type/Name of event
    duration: i32, // How long the even lasts (seconds)
    sps_modifier: i32 // Multiplier that effects overall sps rate 
}

//Implementation's here:
impl Building{
    fn build_building(btype: String, width: i32, height: i32, sps: i32, perk_points: i32)->Building{
        Building { btype, width, height, sps, perk_points}
    }
}
impl Event{
    fn build_event(students_awarded: i32, event_type: String, duration: i32, sps_modifier: i32)->Event{
        Event {students_awarded, event_type, duration, sps_modifier}
    }
}
mod music;
#[macroquad::main("Camera")]
pub async fn main() {
    // let mut test_building = Building::build_building("mainRoom".to_string(), 10, 20, 1, 0);
    // test_building.btype = "building".to_string();
    // println!("{}",test_building.btype);

    // let mut test_event = Event::build_event(1, "Good".to_string(), 100, 1);
    // println!("{}",test_event.students_awarded);
    // test_event.students_awarded = 120;
    // println!("{}",test_event.students_awarded);
    let _music_handle = std::thread::spawn(|| {
        music::music();
    });
    loop {
        gui::gui();

        next_frame().await
    }

}
