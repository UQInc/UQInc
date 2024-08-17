use buildings::*;
use macroquad;
mod gui;
mod music;
mod buildings;
use macroquad::prelude::*;
use music::{music, sound_effect};
use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::thread::spawn;
use std::time::{Duration, Instant};
use std::vec;

struct Building {
    name: &'static str, // The type of building
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
    //Vector containing all buildings
    let sounds = setup_sounds();
    // Use these variables for checking click.
    
    
    let mut buildings: Vec<&'static Building> = Vec::new();
    buildings.push(&FORGANSMITH);
    buildings.push(&GODDARD);
    buildings.push(&PARNELL);
    buildings.push(&RICHARDS);
    buildings.push(&STEELEBUILDING);
    buildings.push(&EZMART);
    buildings.push(&CENTRALLIBRARY);
    buildings.push(&PRENTICE);
    buildings.push(&PRIESTLY);
    buildings.push(&LEARNINGINNOVATION);
    buildings.push(&JOHNHINES);
    buildings.push(&UNIONFOODCOURT);
    buildings.push(&MCELWAIN);
    buildings.push(&CHAMBERLAIN);
    buildings.push(&ARTMUSEUM);
    buildings.push(&OTTO);
    buildings.push(&MOLECULARBIOSCIENCE);
    buildings.push(&JDSTORY);
    buildings.push(&HARTLEY_TEAK);
    buildings.push(&BIO_SCIENCE_LIBRARY);
    buildings.push(&BRAININSTITUTE);
    buildings.push(&WATERANDENVIRO);
    buildings.push(&CHEM);
    buildings.push(&MANSERGHSHAW);
    buildings.push(&HAWKEN);
    buildings.push(&JAMESFOOT);
    buildings.push(&DONNICKLIN);
    buildings.push(&BIOENG);
    buildings.push(&IMAGINGCENTRE);
    buildings.push(&GPSOUTH);
    buildings.push(&GPNORTH);
    buildings.push(&UQBUSINESS);
    buildings.push(&ZELMANCOWEN);
    buildings.push(&BUILDING41);
    buildings.push(&CUMBRAESTEWART);
    buildings.push(&FUNNYNUMBER);
    buildings.push(&OCONNELL);
    buildings.push(&GORDONGREENWOOD);
    buildings.push(&UQCENTRE);
    buildings.push(&BUILDING33);
    buildings.push(&SCHONELLTHEATRE);
    buildings.push(&BRIDGE);
    buildings.push(&PSYCHOLOGY);
    buildings.push(&KATHLEENLAMBOURNE);
    buildings.push(&LIVERIS);
    buildings.push(&ADVENG);

    // Use these variables for checking click.

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

    let font = load_ttf_font("media/fonts/NewAmsterdam-Regular.ttf")
    .await
    .unwrap();

    // Initializes GameState struct
    let mut game_state = start_game();
    let mut notification_manager = gui::NotificationManager::new();
    let textures = load_textures().await;
    let mut time_el = Instant::now();
    let time_req = Duration::from_secs(1);
    loop {
        
        gui::gui(&mut notification_manager, &textures, &game_state);

        let screen_height = screen_height();
        let screen_width = screen_width();
        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                // ImpClick events added for some of the buy menu rectangles.lement functions for the game.
                println!("Game clicked! {} {}", mouse_x, mouse_y);
                game_state.score = clicked(game_state.score);
                sound_effects(String::from("click"), &sounds);
            } else if mouse_x > screen_width * 0.7 {
                // Implement function for buy module.
            }
        }

        gui::build_textdraw(Some(&font));
        gui::perks_textdraw(Some(&font));
        gui::stars_textdraw(Some(&font));


        next_frame().await;
        let duration = time_el.elapsed();
        if (duration >= time_req){
            game_state.score = update_money(game_state.score);
            time_el = Instant::now();
        };
    }

    
}

fn clicked(mut score: Score) -> Score {
    score.curr_students += 1;
    score
}
fn sound_effects(sound: String, sounds: &HashMap<String, PathBuf>) {
    if sound == "click" {
        if let Some(path) = sounds.get("click").cloned() {
            std::thread::spawn(move || {
                music::sound_effect(path, 1);
            });
        }
    }
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

    textures.insert("Test1".to_string(), load_texture("media/images/fortnite_map.png").await.unwrap());

    textures
}


fn setup_sounds() -> HashMap<String, PathBuf> {
    let mut sounds: HashMap<String, PathBuf> = HashMap::new();

    let mut protest = PathBuf::from("media");
    protest.push("sounds");
    protest.push("protest.mp3");
    sounds.insert(String::from("protest"), protest);

    let mut switch = PathBuf::from("media");
    switch.push("sounds");
    switch.push("switch.mp3");
    sounds.insert(String::from("switch"), switch);

    let mut yay = PathBuf::from("media");
    yay.push("sounds");
    yay.push("yay.mp3");
    sounds.insert(String::from("yay"), yay);

    let mut click = PathBuf::from("media");
    click.push("sounds");
    click.push("click.mp3");
    sounds.insert(String::from("click"), click);

    let mut cash = PathBuf::from("media");
    cash.push("sounds");
    cash.push("cash.mp3");
    sounds.insert(String::from("cash"), cash);

    let _music_handle = std::thread::spawn(|| {
        music::music();
    });

    sounds
}

fn update_money(mut score: Score) -> Score{
    if(score.curr_students>0){
        score.dollars += score.curr_students as i32;
    }
    
    score
}

