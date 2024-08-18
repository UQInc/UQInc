use buildings::*;
use gui::{draw_event_gui, draw_event_timer};
use macroquad;
mod gui;
mod music;
mod perks;
mod buildings;
use macroquad::prelude::*;
use music::{music, sound_effect};
use rodio::cpal::available_hosts;
use std::collections::{hash_map, HashMap};
use std::env::set_current_dir;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::thread::{current, spawn};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{default, vec};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};
struct Building {
    name: &'static str, // The type of building
    students: f64,    // Students per Second that this building generates
    perk_points: i32,// Number of perk points awarded by purchasing this building
    price: i64, // Price to purchase building
    description: &'static str,
}

struct Score {
    curr_students: f64, // Current number of available students
    dps: f32,           // Dollars per second value, what is being earned
    dollars: f64,       // Currency in the bank that can be spent
    student_rate: f32,  // The ratio of students/dollar (e.g., 1 student = $2/dps)
    perk_points: i32, // Number of available perk points
}

pub struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String,    // Type/Name of event
    duration: Duration,    // How long the event lasts (seconds)
    dps_modifier: f32,     // Multiplier that affects overall SPS rate
    spc_modifier: f32,
    start_time: Instant,
}

struct GameState {
    score: Score,
    buildings: Vec<&'static Building>, // Active/purchased buildings
    events: Vec<Event>,       // Active events
    start_time: Instant,      // When the game was launched
    stats: Statistics,
    menu_type: String,        // Gameplay stats
    owned_buildings: Vec<&'static Building>,
    available_perks: Vec<&'static Perk>,
    owned_perks: Vec<&'static Perk>,
}

struct Statistics {
    total_students: u64, // Number of students ever earned
    total_clicks: u64,   // Number of clicks ever registered
    total_events: u64,   // Number of events that have occurred
}

struct Perk {
    dps_modifier: f32,
    student_rate: f32,
    name: &'static str,
    price: i64,
    description: &'static str,
}

// Implementations here:
impl Score {
    fn init() -> Self {
        Score {
            curr_students: 0.,
            dps: 1.0,
            dollars: 0.,
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
        dps_modifier: f32,
        spc_modifier: f32,
    ) -> Event {
        let duration = Duration::new(duration, 0);
        Event {
            students_awarded,
            event_type,
            duration,
            dps_modifier,
            spc_modifier,
            start_time: Instant::now(),
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            students_awarded: 0,
            event_type: "".to_string(),
            duration: Duration::from_secs(30),
            dps_modifier: 1.,
            spc_modifier: 1.,
            start_time: Instant::now(),
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
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    //Vector containing all buildings
    let sounds = setup_sounds();
    // Use these variables for checking click.
    let _music_handle = std::thread::spawn(|| {
        music::music();
    });
    
    let mut buildings: Vec<&'static Building> = Vec::new();
    buildings.push(&FORGANSMITH);
    buildings.push(&GODDARD);
    buildings.push(&PARNELL);
    buildings.push(&RICHARDS);
    buildings.push(&STEELEBUILDING);
    buildings.push(&EZMART);
    buildings.push(&CENTRALLIBRARY);
    buildings.push(&PRIESTLY);
    buildings.push(&LEARNINGINNOVATION);
    buildings.push(&JOHNHINES);
    buildings.push(&UNIONFOODCOURT);
    buildings.push(&MCELWAIN);
    buildings.push(&CHAMBERLAIN);
    buildings.push(&ARTMUSEUM);
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
    buildings.push(&PSYCHOLOGY);
    buildings.push(&KATHLEENLAMBOURNE);
    buildings.push(&ADVENG);
    buildings.push(&LIVERIS);


    let mut perks: Vec<&'static Perk> = Vec::new();


    let mut owned_buildings: Vec<&'static Building> = Vec::new();
    let mut owned_perks: Vec<&'static Perk> = Vec::new();
    
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
    let mut game_state = start_game(buildings, owned_buildings, owned_perks, perks);

    let textures = load_textures().await;
    let mut time_el = Instant::now();
    let time_req = Duration::from_secs(1);
    let mut last_event_time = Instant::now();
    let mut current_event: Option<Event> = None;
    let mut draw_event_popup: bool = false;

    // Seed random based on system time
    rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let label_style = root_ui()
        .style_builder()
        .font(include_bytes!("NewAmsterdam-Regular.ttf"))
        .unwrap()
        .font_size(20)
        .build();
    let currency_skin = 
        Skin {
            label_style,
            ..root_ui().default_skin()
        };

    // Draw currency widget with custom font
    root_ui().push_skin(&currency_skin);
    loop {

        gui::gui(&textures, &mut game_state, Some(&font));

        let screen_height = screen_height();
        let screen_width = screen_width();
        let font_size = dynamic_font_size(60.0);

        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                // ImpClick events added for some of the buy menu rectangles.lement functions for the game.
                game_state.score = clicked(game_state.score, current_event.as_ref());
                if let Some(path) = sounds.get("click").cloned() {
                    std::thread::spawn(move || {
                        music::sound_effect(path, 1);
                    });
                }
            } else if mouse_x > screen_width * 0.7 {
                // Implement function for buy module.
            }
        }

        gui::build_textdraw(Some(&font), font_size);
        gui::perks_textdraw(Some(&font), font_size);

        if (game_state.menu_type == "build") {
            if let Some(current_building_0) = game_state.buildings.get(0) {
                gui::buymenu_font(Some(&font), font_size, current_building_0.name.to_string(), 0);
                gui::buymenu_price(Some(&font), font_size, current_building_0.price, 0);
                gui::buymenu_description(Some(&font), font_size, current_building_0.description.to_string(), 0);
            }
            if let Some(current_building_1) = game_state.buildings.get(1) {
                gui::buymenu_font(Some(&font), font_size, current_building_1.name.to_string(), 1);
                gui::buymenu_price(Some(&font), font_size, current_building_1.price, 1);
                gui::buymenu_description(Some(&font), font_size, current_building_1.description.to_string(), 1);
            }

            if let Some(current_building_2) = game_state.buildings.get(2) {
                gui::buymenu_font(Some(&font), font_size, current_building_2.name.to_string(), 2);
                gui::buymenu_price(Some(&font), font_size, current_building_2.price, 2);
                gui::buymenu_description(Some(&font), font_size, current_building_2.description.to_string(), 2);
            }

            if let Some(current_building_3) = game_state.buildings.get(3) {
                gui::buymenu_font(Some(&font), font_size, current_building_3.name.to_string(), 3);
                gui::buymenu_price(Some(&font), font_size, current_building_3.price, 3);
                gui::buymenu_description(Some(&font), font_size, current_building_3.description.to_string(), 3);
            }

            if let Some(current_building_4) = game_state.buildings.get(4) {
                gui::buymenu_font(Some(&font), font_size, current_building_4.name.to_string(), 4);
                gui::buymenu_price(Some(&font), font_size, current_building_4.price, 4);
                gui::buymenu_description(Some(&font), font_size, current_building_4.description.to_string(), 4);
            }
        } else {
            if let Some(current_perk_0) = game_state.available_perks.get(0) {
                gui::buymenu_font(Some(&font), font_size, current_perk_0.name.to_string(), 0);
                gui::buymenu_price(Some(&font), font_size, current_perk_0.price, 0);
                gui::buymenu_description(Some(&font), font_size, current_perk_0.description.to_string(), 0);
            }

            if let Some(current_perk_1) = game_state.available_perks.get(1) {
                gui::buymenu_font(Some(&font), font_size, current_perk_1.name.to_string(), 1);
                gui::buymenu_price(Some(&font), font_size, current_perk_1.price, 1);
                gui::buymenu_description(Some(&font), font_size, current_perk_1.description.to_string(), 1);
            }

            if let Some(current_perk_2) = game_state.available_perks.get(2) {
                gui::buymenu_font(Some(&font), font_size, current_perk_2.name.to_string(), 2);
                gui::buymenu_price(Some(&font), font_size, current_perk_2.price, 2);
                gui::buymenu_description(Some(&font), font_size, current_perk_2.description.to_string(), 2);
            }

            if let Some(current_perk_3) = game_state.available_perks.get(3) {
                gui::buymenu_font(Some(&font), font_size, current_perk_3.name.to_string(), 3);
                gui::buymenu_price(Some(&font), font_size, current_perk_3.price, 3);
                gui::buymenu_description(Some(&font), font_size, current_perk_3.description.to_string(), 3);
            }

            if let Some(current_perk_4) = game_state.available_perks.get(4) {
                gui::buymenu_font(Some(&font), font_size, current_perk_4.name.to_string(), 4);
                gui::buymenu_price(Some(&font), font_size, current_perk_4.price, 4);
                gui::buymenu_description(Some(&font), font_size, current_perk_4.description.to_string(), 4);
            }
        }

        next_frame().await;
        let duration = time_el.elapsed();
        if (duration >= time_req){
            game_state.score = update_money(game_state.score, current_event.as_ref());
            time_el = Instant::now();
        };

        //Draw event timer
        if current_event.as_ref().is_some() {
            draw_event_timer(current_event.as_ref().unwrap());
        }

        if draw_event_popup {
            if current_event.as_ref().is_some() {
                draw_event_popup = draw_event_gui(current_event.as_ref().unwrap());
            }
        }

        // Check if ready for an event roll, if ready, roll for an event and add the new event.
        if last_event_time.elapsed() >= Duration::from_secs(5) {
            println!("Rolling for event");

            last_event_time = Instant::now();
            if !current_event.as_ref().is_some() {
                let event = get_event_from_rand(rand::gen_range(0, 20), &game_state);
                if event.is_some() {
                    println!("New Event Added");
                    current_event = event;

                    println!("{}", current_event.as_ref().unwrap().event_type);

                    draw_event_popup = gui::draw_event_gui(current_event.as_ref().unwrap());
    
                    if current_event.as_ref().unwrap().event_type == "AddStudents" {
                        game_state.score.curr_students += current_event.as_ref().unwrap().students_awarded as f64;
                    }
                }
            }
        }

        if current_event.as_ref().is_some() {
            if current_event.as_ref().unwrap().duration.as_secs() < current_event.as_ref().unwrap().start_time.elapsed().as_secs() {
                current_event = None;
                println!("Removing Event");
            }
        }
    }
}

fn get_event_from_rand(num: i32, state: &GameState) -> Option<Event>{
    if num == 0 {
        // Inflation
        let out = Event {
            dps_modifier: 2.,
            event_type: "CashProduction".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    } else if num == 1 {
        // Good press
        let out = Event {
            students_awarded: (state.score.curr_students as f32 * 0.05) as i32,
            event_type: "AddStudents".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    } else if num == 2 {
        // Ranking increased
        let out = Event {
            spc_modifier: 2.,
            event_type: "StudentsPerClick".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    } else if num == 3 {
        // International sanctions
        let out = Event {
            dps_modifier: 0.5,
            event_type: "CashProduction".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    } else if num == 4 {
        // Bad Press
        let out = Event {
            students_awarded: ((state.score.curr_students as f32 * 0.05) * -1.) as i32,
            event_type: "AddStudents".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    } else if num == 5 {
        // Pandemic
        let out = Event {
            spc_modifier: 0.5,
            event_type: "StudentsPerClick".to_string(),
            ..Default::default()
        };
        return Option::from(out);
    }

    return None;
}

fn clicked(mut score: Score, event: Option<&Event>) -> Score {
    // Get the current event multiplier
    let mut mult = 1.;
    if event.is_some() {
        mult = event.unwrap().spc_modifier;
    }
    score.curr_students += (1. * mult) as f64;
    score
}

fn start_game(unown: Vec<&'static Building>,
    owned: Vec<&'static Building>, owned_perks: Vec<&'static Perk>, 
    available_perks: Vec<&'static Perk>) -> GameState {
    let score: Score = Score::init();
    let buildings = unown;
    let owned_buildings: Vec<&Building> = owned;
    let events: Vec<Event> = Vec::new();
    let start_time = Instant::now();
    let stats: Statistics = Statistics::init();
    let menu_type: String = "build".to_string();
    let owned_perks: Vec<&Perk> = owned_perks;
    let available_perks: Vec<&'static Perk> = available_perks;
    
    GameState {
        score,
        buildings,
        events,
        start_time,
        stats,
        menu_type,
        owned_buildings,
        owned_perks,
        available_perks,
    }
}

async fn load_textures() -> HashMap<String, Texture2D> {
    let buildings = [
        ("Background", "media/images/BACKGROUND.png"),
        ("Forgan Smith", "media/images/FORGANSMITH.png"),
        ("Goddard Building", "media/images/GODDARD.png"),
        ("Parnell Building", "media/images/PARNELL.png"),
        ("Richards Building", "media/images/RICHARDS.png"),
        ("Steele Building", "media/images/STEELEBUILDING.png"),
        ("EZ Mart", "media/images/EZMART.png"),
        ("Central Library", "media/images/CENTRALLIBRARY.png"),
        ("Priestly Building", "media/images/PRIESTLY.png"),
        ("Learning Innovation", "media/images/LEARNINGINNOVATION.png"),
        ("John Hines Building", "media/images/JOHNHINES.png"),
        ("UQ Union Food Court", "media/images/UNIONFOODCOURT.png"),
        ("McElwain Building", "media/images/MCELWAIN.png"),
        ("Chamberlain Building", "media/images/CHAMBERLAIN.png"),
        ("Art Museum", "media/images/ARTMUSEUM.png"),
        ("Otto Hirschfeld Building", "media/images/RICHARDS.png"),
        ("Molecular BioScience", "media/images/MOLECULARBIOSCIENCE.png"),
        ("JD Story Admin", "media/images/JDSTORY.png"),
        ("Hartley Teak", "media/images/HARTLEY_TEAK.png"),
        ("Biological Library", "media/images/BIO_SCIENCE_LIBRARY.png"),
        ("Brain Institution", "media/images/BRAININSTITUTE.png"),
        ("Env Biotechnology", "media/images/WATERANDENVIRO.png"),
        ("Chemistry Building", "media/images/CHEM.png"),
        ("M. Shaw Building", "media/images/MANSERGHSHAW.png"),
        ("Hawken Engineering", "media/images/HAWKEN.png"),
        ("Sir J. Foot Building", "media/images/JAMESFOOT.png"),
        ("D. Nicklin Building", "media/images/DONNICKLIN.png"),
        ("Bioeng Institute", "media/images/BIOENG.png"),
        ("Imaging Centre", "media/images/IMAGINGCENTRE.png"),
        ("GP South", "media/images/GPSOUTH.png"),
        ("GP North", "media/images/GPNORTH.png"),
        ("UQ Business School", "media/images/UQBUSINESS.png"),
        ("Z. Cowen Building", "media/images/ZELMANCOWEN.png"),
        ("Building 41", "media/images/BUILDING41.png"),
        ("23, 38, 31A", "media/images/FUNNYNUMBER.png"),
        ("Cumbrae-Stewart", "media/images/CUMBRAESTEWART.png"),
        ("O'Connell Building", "media/images/OCONNELL.png"),
        ("G. Greenwood Building", "media/images/GORDONGREENWOOD.png"),
        ("UQ Centre", "media/images/UQCENTRE.png"),
        ("Building 33", "media/images/BUILDING33.png"),
        ("Schonell Theatre", "media/images/SCHONELLTHEATRE.png"),
        ("Psychology Building", "media/images/PSYCHOLOGY.png"),
        ("K. Lambourne Building", "media/images/KATHLEENLAMBOURNE.png"),
        ("Advanced Engineering", "media/images/ADVENG.png"),
        ("Andrew N. Liveris", "media/images/LIVERIS.png"),
        ("Foreground", "media/images/FOREGROUND.png"),
        ("BuyIcon", "media/images/BUILDINGICON.png"),
    ];

    let mut textures = HashMap::new();
    for (name, path) in &buildings {
        textures.insert(name.to_string(), load_texture(path).await.unwrap());
    }

    textures
}

pub fn setup_sounds() -> HashMap<String, PathBuf> {
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



    sounds
}

fn update_money(mut score: Score, event: Option<&Event>) -> Score{
    // Get the current event multiplier
    let mut mult = 1.;
    if event.is_some() {
        mult = event.unwrap().dps_modifier as f64;
    }

    if score.curr_students > 0. {
        score.dollars += score.curr_students * mult;
    }
    
    score
}

fn dynamic_font_size(base_font_size: f32) -> u16 {
    let reference_width = 1920.0;
    let reference_height = 1080.0;

    let screen_width = screen_width();
    let screen_height = screen_height();

    // Calculate scaling factors for width and height
    let width_scale = screen_width / reference_width;
    let height_scale = screen_height / reference_height;

    // Use the smaller scale to maintain aspect ratio
    let scale_factor = width_scale.min(height_scale);

    let size = base_font_size * scale_factor;
    size as u16
}
