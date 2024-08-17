use macroquad;
mod gui;
mod music;
use macroquad::prelude::*;
use music::{music, sound_effect};
use std::collections::{hash_map, HashMap};
use std::thread::spawn;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use std::vec;

struct Building {
    btype: String, // The type of building 
    width: i32,
    height: i32,
    sps: i32, // Students per Second that this building generates
    perk_points: i32, // Number of perk points awarded by purchasing this building
}

struct Score {
    students: i32, // Total number of students cummulated
    currStudents: i32, // Current number of avaliable students
    dps: i32, // Dollars per second value, what is being earnt
    dollars: i32, // Currency
}

struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String, // Type/Name of event
    duration: i32, // How long the even lasts (seconds)
    dps_modifier: i32 // Multiplier that effects overall sps rate 
}

struct GameState {
    score: Score,
    buildings: Vec<Building>,
    events: Vec<Event>,
    start_time: Instant,
}

//Implementation's here:
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

impl Building{
    fn build_building(btype: String, width: i32, height: i32, sps: i32, perk_points: i32)->Building{
        Building { btype, width, height, sps, perk_points}
    }
}
impl Event{
    fn build_event(students_awarded: i32, event_type: String, duration: i32, dps_modifier: i32)->Event{
        Event {students_awarded, event_type, duration, dps_modifier}
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
    // Use these variables for checking click.
    let screen_height = screen_height();
    let screen_width = screen_width();
    // let mut test_building = Building::build_building("mainRoom".to_string(), 10, 20, 1, 0);
    // test_building.btype = "building".to_string();
    // println!("{}",test_building.btype);
    // let mut test_event = Event::build_event(1, "Good".to_string(), 100, 1);
    // println!("{}",test_event.students_awarded);
    // test_event.students_awarded = 120;
    // println!("{}",test_event.students_awarded);

    let sounds = setup_sounds();
    // for (key, value) in sounds {
    //     let _sound_handle = std::thread::spawn(move || {
    //         music::sound_effect(value, 1)
    //     });
    //     _sound_handle.join().unwrap();
    // }
    
    // let _sound_handle = std::thread::spawn(move || {
    //     if let Some(protest) = sounds.get("protest").cloned() {
    //         music::sound_effect(protest, 1)
    //     }
    // });

    start_game();
    loop {
        gui::gui();
        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x,mouse_y) = mouse_position();
            if (mouse_x < screen_width * 0.7) {
                // Implement functions for game.
                println!("Game clicked! {} {}", mouse_x, mouse_y);
            }
            if (mouse_x > screen_width * 0.7) {
                // Impmement function for buy module.
                println!("Buy module clicked {} {}", mouse_x, mouse_y);
            }
       }
        
        next_frame().await
    }
}

fn start_game() -> Score {
    let mut score: Score = Score::init();
    score
}

fn setup_sounds() -> HashMap<String, PathBuf> {
    let mut sounds: HashMap<String, PathBuf> = HashMap::new();

    let mut protest = PathBuf::from("src");
    protest.push("media");
    protest.push("sounds");
    protest.push("protest.mp3");
    sounds.insert(String::from("protest"), protest);

    let mut switch = PathBuf::from("src");
    switch.push("media");
    switch.push("sounds");
    switch.push("switch.mp3");
    sounds.insert(String::from("switch"), switch);

    let mut yay = PathBuf::from("src");
    yay.push("media");
    yay.push("sounds");
    yay.push("yay.mp3");
    sounds.insert(String::from("yay"), yay);

    let mut click = PathBuf::from("src");
    click.push("media");
    click.push("sounds");
    click.push("click.mp3");
    sounds.insert(String::from("click"), click);

    let mut cash = PathBuf::from("src");
    cash.push("media");
    cash.push("sounds");
    cash.push("cash.mp3");
    sounds.insert(String::from("cash"), cash);
    
    // let _music_handle = std::thread::spawn(|| {
    //     music::music();
    // });

    sounds
}