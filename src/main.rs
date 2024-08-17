use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod gui;
mod music;

struct Building {
    btype: String, // The type of building
    width: i32,
    height: i32,
    students: i32,    // Students per Second that this building generates
    perk_points: i32, // Number of perk points awarded by purchasing this building
}

struct Score {
    curr_students: i32, // Current number of available students
    dps: f32,           // Dollars per second value, what is being earned
    dollars: i32,       // Currency in the bank that can be spent
    student_rate: f32,  // The ratio of students/dollar (e.g., 1 student = $2/dps)
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
        }
    }

    fn build_score(curr_students: i32, dps: f32, dollars: i32) -> Self {
        Score {
            curr_students,
            dps,
            dollars,
            student_rate: 1.0, // Default value or calculate based on other parameters
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
        btype: String,
        width: i32,
        height: i32,
        students: i32,
        perk_points: i32,
    ) -> Building {
        Building {
            btype,
            width,
            height,
            students,
            perk_points,
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

    let _music_handle = std::thread::spawn(|| {
        music::music();
    });
    let _sound_handle = std::thread::spawn(|| {
        music::sound_effect("src\\media\\sounds\\click.mp3", 1);
    });

    // Initializes GameState struct
    let mut game_state = start_game();

    loop {
        gui::gui(); // Pass score to gui function

        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                // Implement functions for the game.
                println!("Game clicked! {} {}", mouse_x, mouse_y);
                game_state.score = clicked(game_state.score);
            } else if mouse_x > screen_width * 0.7 {
                // Implement function for buy module.
                println!("Buy module clicked {} {}", mouse_x, mouse_y);
            }
        }

        next_frame().await
    }
}

fn clicked(score: Score) -> Score {
    let new_score = Score::build_score(score.curr_students + 1, score.dps, score.dollars);
    new_score
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
