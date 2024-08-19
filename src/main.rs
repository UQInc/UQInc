use buildings::*;
use perks::*;
use gui::{draw_event_gui, draw_event_timer};
use macroquad::prelude::*;
use music::{music, sound_effect, sound_effect_from_bytes};
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use macroquad::ui::{root_ui, Skin};

mod gui;
mod music;
mod perks;
mod buildings;

// Embed the assets directory
#[derive(RustEmbed)]
#[folder = "media/"] // Specify the directory with your assets
struct Asset;

struct Building {
    name: &'static str, // The type of building
    students: f64,      // Students per Second that this building generates
    perk_points: i32,   // Number of perk points awarded by purchasing this building
    price: i64,         // Price to purchase building
    description: &'static str,
}

struct Score {
    curr_students: f64, // Current number of available students
    dps: f32,           // Dollars per second value, what is being earned
    dollars: f64,       // Currency in the bank that can be spent
    student_rate: f32,  // The ratio of students/dollar (e.g., 1 student = $2/dps)
    perk_points: i32,   // Number of available perk points
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
    // Vector containing all buildings
    let sounds = setup_sounds();
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

    perks.push(&INCREASEFEES1);
    perks.push(&INCREASEFEES2);
    perks.push(&INCREASEFEES3);
    perks.push(&INCREASEFEES4);
    perks.push(&INCREASECLICKS1);
    perks.push(&INCREASECLICKS2);
    perks.push(&INCREASECLICKS3);
    perks.push(&INCREASECLICKS4);
    perks.push(&INCREASECLICKS5);

    let mut owned_buildings: Vec<&'static Building> = Vec::new();
    let mut owned_perks: Vec<&'static Perk> = Vec::new();

    let font_data = Asset::get("fonts/NewAmsterdam-Regular.ttf").unwrap();
    let font = load_ttf_font_from_bytes(&font_data.data).unwrap();

    let mut game_state = start_game(buildings, owned_buildings, owned_perks, perks);

    let textures = load_textures().await;
    let mut time_el = Instant::now();
    let time_req = Duration::from_secs(1);
    let mut last_event_time = Instant::now();
    let mut current_event: Option<Event> = None;
    let mut draw_event_popup: bool = false;

    rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let font_data = Asset::get("fonts/NewAmsterdam-Regular.ttf").unwrap();
    let label_style = root_ui()
        .style_builder()
        .font(&font_data.data) // Pass the raw bytes here
        .unwrap()
        .font_size(20)
        .build();
    let currency_skin = Skin {
        label_style,
        ..root_ui().default_skin()
    };


    root_ui().push_skin(&currency_skin);
    loop {
        gui::gui(&textures, &mut game_state, Some(&font));

        let screen_height = screen_height();
        let screen_width = screen_width();
        let font_size = dynamic_font_size(60.0);


        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                game_state.score = clicked(game_state.score, current_event.as_ref());
                if let Some(sound_data) = sounds.get("click").cloned() {
                    // Spawn a new thread to play the sound effect without blocking the main thread
                    std::thread::spawn(move || {
                        music::sound_effect_from_bytes(sound_data, 1);
                    });
                }
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
                gui::buymenu_description(Some(&font), font_size, current_perk_0.description.to_string(), 0);
            }

            if let Some(current_perk_1) = game_state.available_perks.get(1) {
                gui::buymenu_font(Some(&font), font_size, current_perk_1.name.to_string(), 1);
                gui::buymenu_description(Some(&font), font_size, current_perk_1.description.to_string(), 1);
            }

            if let Some(current_perk_2) = game_state.available_perks.get(2) {
                gui::buymenu_font(Some(&font), font_size, current_perk_2.name.to_string(), 2);
                gui::buymenu_description(Some(&font), font_size, current_perk_2.description.to_string(), 2);
            }

            if let Some(current_perk_3) = game_state.available_perks.get(3) {
                gui::buymenu_font(Some(&font), font_size, current_perk_3.name.to_string(), 3);
                gui::buymenu_description(Some(&font), font_size, current_perk_3.description.to_string(), 3);
            }

            if let Some(current_perk_4) = game_state.available_perks.get(4) {
                gui::buymenu_font(Some(&font), font_size, current_perk_4.name.to_string(), 4);
                gui::buymenu_description(Some(&font), font_size, current_perk_4.description.to_string(), 4);
            }
        }

        next_frame().await;

        let duration = time_el.elapsed();
        if duration >= time_req {
            game_state.score = update_money(game_state.score, current_event.as_ref());
            time_el = Instant::now();
        }

        if let Some(event) = &current_event {
            draw_event_timer(event);
            if event.duration.as_secs() < event.start_time.elapsed().as_secs() {
                current_event = None;
            }
        }

        if draw_event_popup {
            if let Some(event) = &current_event {
                draw_event_popup = draw_event_gui(event);
            }
        }

        if last_event_time.elapsed() >= Duration::from_secs(5) {
            last_event_time = Instant::now();
            if current_event.is_none() {
                if let Some(event) = get_event_from_rand(rand::gen_range(0, 20), &game_state) {
                    current_event = Some(event);
                    draw_event_popup = draw_event_gui(&current_event.as_ref().unwrap());
                    if current_event.as_ref().unwrap().event_type == "AddStudents" {
                        game_state.score.curr_students += current_event.as_ref().unwrap().students_awarded as f64;
                    }
                }
            }
        }
    }
}
async fn load_textures() -> HashMap<String, Texture2D> {
    let buildings = [
        ("Background", "images/BACKGROUND.png"),
        ("Forgan Smith", "images/FORGANSMITH.png"),
        ("Goddard Building", "images/GODDARD.png"),
        ("Parnell Building", "images/PARNELL.png"),
        ("Richards Building", "images/RICHARDS.png"),
        ("Steele Building", "images/STEELEBUILDING.png"),
        ("EZ Mart", "images/EZMART.png"),
        ("Central Library", "images/CENTRALLIBRARY.png"),
        ("Priestly Building", "images/PRIESTLY.png"),
        ("Learning Innovation", "images/LEARNINGINNOVATION.png"),
        ("John Hines Building", "images/JOHNHINES.png"),
        ("UQ Union Food Court", "images/UNIONFOODCOURT.png"),
        ("McElwain Building", "images/MCELWAIN.png"),
        ("Chamberlain Building", "images/CHAMBERLAIN.png"),
        ("Art Museum", "images/ARTMUSEUM.png"),
        ("Otto Hirschfeld Building", "images/RICHARDS.png"),
        ("Molecular BioScience", "images/MOLECULARBIOSCIENCE.png"),
        ("JD Story Admin", "images/JDSTORY.png"),
        ("Hartley Teak", "images/HARTLEY_TEAK.png"),
        ("Biological Library", "images/BIO_SCIENCE_LIBRARY.png"),
        ("Brain Institution", "images/BRAININSTITUTE.png"),
        ("Env Biotechnology", "images/WATERANDENVIRO.png"),
        ("Chemistry Building", "images/CHEM.png"),
        ("M. Shaw Building", "images/MANSERGHSHAW.png"),
        ("Hawken Engineering", "images/HAWKEN.png"),
        ("Sir J. Foot Building", "images/JAMESFOOT.png"),
        ("D. Nicklin Building", "images/DONNICKLIN.png"),
        ("Bioeng Institute", "images/BIOENG.png"),
        ("Imaging Centre", "images/IMAGINGCENTRE.png"),
        ("GP South", "images/GPSOUTH.png"),
        ("GP North", "images/GPNORTH.png"),
        ("UQ Business School", "images/UQBUSINESS.png"),
        ("Z. Cowen Building", "images/ZELMANCOWEN.png"),
        ("Building 41", "images/BUILDING41.png"),
        ("23, 38, 31A", "images/FUNNYNUMBER.png"),
        ("Cumbrae-Stewart", "images/CUMBRAESTEWART.png"),
        ("O'Connell Building", "images/OCONNELL.png"),
        ("G. Greenwood Building", "images/GORDONGREENWOOD.png"),
        ("UQ Centre", "images/UQCENTRE.png"),
        ("Building 33", "images/BUILDING33.png"),
        ("Schonell Theatre", "images/SCHONELLTHEATRE.png"),
        ("Psychology Building", "images/PSYCHOLOGY.png"),
        ("K. Lambourne Building", "images/KATHLEENLAMBOURNE.png"),
        ("Advanced Engineering", "images/ADVENG.png"),
        ("Andrew N. Liveris", "images/LIVERIS.png"),
        ("Foreground", "images/FOREGROUND.png"),
        ("BuyIcon", "images/BUILDINGICON.png"),
    ];
    let mut textures = HashMap::new();
    for (name, path) in &buildings {
        if let Some(texture_data) = Asset::get(path) {
            let texture = Texture2D::from_file_with_format(&texture_data.data, None);
            textures.insert(name.to_string(), texture);
        } else {
            println!("Warning: Could not find asset for {}", path);
        }
    }

    textures
}
pub fn setup_sounds() -> HashMap<String, Vec<u8>> {
    let mut sounds: HashMap<String, Vec<u8>> = HashMap::new();

    let protest = Asset::get("sounds/protest.mp3").unwrap();
    sounds.insert("protest".to_string(), protest.data.to_vec());

    let switch = Asset::get("sounds/switch.mp3").unwrap();
    sounds.insert("switch".to_string(), switch.data.to_vec());

    let yay = Asset::get("sounds/yay.mp3").unwrap();
    sounds.insert("yay".to_string(), yay.data.to_vec());

    let click = Asset::get("sounds/click.mp3").unwrap();
    sounds.insert("click".to_string(), click.data.to_vec());

    let cash = Asset::get("sounds/cash.mp3").unwrap();
    sounds.insert("cash".to_string(), cash.data.to_vec());

    sounds
}

fn update_money(mut score: Score, event: Option<&Event>) -> Score {
    let mut mult = 1.;
    if let Some(event) = event {
        mult = event.dps_modifier as f64;
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

    let width_scale = screen_width / reference_width;
    let height_scale = screen_height / reference_height;

    let scale_factor = width_scale.min(height_scale);

    let size = base_font_size * scale_factor;
    size as u16
}

fn get_event_from_rand(num: i32, state: &GameState) -> Option<Event> {
    match num {
        0 => Some(Event {
            dps_modifier: 2.,
            event_type: "CashProduction".to_string(),
            ..Default::default()
        }),
        1 => Some(Event {
            students_awarded: (state.score.curr_students as f32 * 0.05) as i32,
            event_type: "AddStudents".to_string(),
            ..Default::default()
        }),
        2 => Some(Event {
            spc_modifier: 2.,
            event_type: "StudentsPerClick".to_string(),
            ..Default::default()
        }),
        3 => Some(Event {
            dps_modifier: 0.5,
            event_type: "CashProduction".to_string(),
            ..Default::default()
        }),
        4 => Some(Event {
            students_awarded: ((state.score.curr_students as f32 * 0.05) * -1.) as i32,
            event_type: "AddStudents".to_string(),
            ..Default::default()
        }),
        5 => Some(Event {
            spc_modifier: 0.5,
            event_type: "StudentsPerClick".to_string(),
            ..Default::default()
        }),
        _ => None,
    }
}

fn clicked(mut score: Score, event: Option<&Event>) -> Score {
    let mut mult = 1.;
    if let Some(event) = event {
        mult = event.spc_modifier;
    }
    score.curr_students += (1. * mult) as f64;
    score
}

fn start_game(
    unown: Vec<&'static Building>,
    owned: Vec<&'static Building>,
    owned_perks: Vec<&'static Perk>,
    available_perks: Vec<&'static Perk>,
) -> GameState {
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

