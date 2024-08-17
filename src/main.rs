use macroquad;
mod gui;
mod music;
use macroquad::audio::Sound;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

struct Building {
    btype: String, // The type of building 
    width: i32,
    height: i32,
    students: i32, // Students per Second that this building generates
    perk_points: i32, // Number of perk points awarded by purchasing this building
}

struct Score {
    currStudents: i32, // Current number of avaliable students
    dps: f32, // Dollars per second value, what is being earnt
    dollars: i32, // Currency in the bank that can be spent
    student_rate: f32 // The ratio of students/dollar (e.g. 1 student = $2/dps)
}

struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String, // Type/Name of event
    duration: Duration, // How long the even lasts (seconds)
    dps_modifier: i32 // Multiplier that effects overall sps rate 
}

struct GameState {
    score: Score, 
    buildings: Vec<Building>, // Active/purchased buildings
    events: Vec<Event>, // Active events
    start_time: Instant, // When the game was launched
    stats: Statistics, // Gameplay stats
}

struct Statistics {
    total_students: u64, // Number of students ever earnt
    total_clicks: u64, // Number of clicks ever registered
    total_events: u64, // Number of events that have occurred
}

//Implementation's here:
impl Score {
    fn init() -> Self {
        Score {
            currStudents: 0,
            dps: 1.0,
            dollars: 0,
            student_rate: 1.0
        }
    }

    fn calc_dps(&self, modifiers: Vec<f32>) -> f32 {
        let mut new_sps: f32 = self.dps;
        for i in modifiers.iter() {
            new_sps *= i;
        }
        new_sps
    }

}

impl Building{
    fn build_building(btype: String, width: i32, height: i32, students: i32, perk_points: i32)->Building{
        Building { btype, width, height, students, perk_points}
    }
}

impl Event{
    fn build_event(students_awarded: i32, event_type: String, duration: u64, dps_modifier: i32)->Event{
        let duration = Duration::new(duration, 0);
        Event {students_awarded, event_type, duration, dps_modifier}
    }
}

impl Statistics {
    fn init() -> Self {
        Statistics {
            total_students: 0,
            total_clicks: 0,
            total_events: 0,
        }  
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "UQ, Inc.".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
    let _sound_handle = std::thread::spawn(|| {
        music::sound_effect("src\\media\\sounds\\click.mp3", 1);
    });
    start_game();
    loop {
        gui::gui();
        next_frame().await
    }
}

fn start_game() -> GameState {
    let mut score: Score = Score::init();
    let mut buildings: Vec<Building> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let start_time = Instant::now();
    let mut stats: Statistics = Statistics::init();
    GameState {
        score,
        buildings,
        events,
        start_time,
        stats,
    }
}
