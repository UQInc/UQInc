use macroquad;
mod gui;
use macroquad::prelude::*;
use music::{music, sound_effect};
use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::thread::spawn;
use std::time::{Duration, Instant};
use std::vec;
mod music;

struct Building {
    name: String, // The type of building
    students: i32,    // Students per Second that this building generates
    perk_points: i32,// Number of perk points awarded by purchasing this building
    price: i32, // Price to purchase building
}

struct Score {
    curr_students: i32, // Current number of available students
    dps: f32,           // Dollars per second value, what is being earned
    dollars: i32,       // Currency in the bank that can be spent
    student_rate: f32,  // The ratio of students/dollar (e.g., 1 student = $2/dps)
    perk_points: i32, // Number of available perk points
}

struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String,    // Type/Name of event
    duration: Duration,    // How long the event lasts (seconds)
    dps_modifier: i32,     // Multiplier that affects overall SPS rate
}

struct GameState {
    score: Score,
    buildings: Vec<Building>, // Active/purchased buildings
    events: Vec<Event>,       // Active events
    start_time: Instant,      // When the game was launched
    stats: Statistics,        // Gameplay stats
}

struct Statistics {
    total_students: u64, // Number of students ever earned
    total_clicks: u64,   // Number of clicks ever registered
    total_events: u64,   // Number of events that have occurred
}

// Implementations here:
impl Score {
    fn init() -> Self {
        Score {
            curr_students: 0,
            dps: 1.0,
            dollars: 0,
            student_rate: 1.0,
            perk_points: 0,

        }
    }

    fn calc_dps(&self, modifiers: Vec<f32>) -> f32 {
        let mut new_dps: f32 = self.dps;
        for i in modifiers.iter() {
            new_dps *= i;
        }
        new_dps
    }
}

impl Building {
    fn build_building(
        name: String,
        students: i32,
        perk_points: i32,
        price: i32,
    ) -> Building {
        Building {
            name,
            students,
            perk_points,
            price,
        }
    }
}

impl Event {
    fn build_event(
        students_awarded: i32,
        event_type: String,
        duration: u64,
        dps_modifier: i32,
    ) -> Event {
        let duration = Duration::new(duration, 0);
        Event {
            students_awarded,
            event_type,
            duration,
            dps_modifier,
        }
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
    // Use these variables for checking click.
    let screen_height = screen_height();
    let screen_width = screen_width();
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

    // Initializes GameState struct
    let mut game_state = start_game();
    let mut notification_manager = gui::NotificationManager::new();
    let textures = load_textures().await;

    loop {
        gui::gui(&mut notification_manager, &textures, &game_state);

        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                // ImpClick events added for some of the buy menu rectangles.lement functions for the game.
                println!("Game clicked! {} {}", mouse_x, mouse_y);
                game_state.score = clicked(game_state.score);
            } else if mouse_x > screen_width * 0.7 {
                // Implement function for buy module.
            }
        }

        next_frame().await
    }

    
}

fn clicked(mut score: Score) -> Score {
    score.curr_students += 1;
    score
}

fn start_game() -> GameState {
    let score: Score = Score::init();
    let buildings: Vec<Building> = Vec::new();
    let events: Vec<Event> = Vec::new();
    let start_time = Instant::now();
    let stats: Statistics = Statistics::init();
    GameState {
        score,
        buildings,
        events,
        start_time,
        stats,
    }
}

async fn load_textures() -> HashMap<String, Texture2D> {
    let mut textures = HashMap::new();

    textures.insert("Test1".to_string(), load_texture("src/media/images/fortnite_map.png").await.unwrap());

    textures
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

    let _music_handle = std::thread::spawn(|| {
        music::music();
    });

    sounds
}

