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
    students: i32,    // Students per Second that this building generates
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

struct Event {
    students_awarded: i32, // Number of students this event gives (can be negative)
    event_type: String,    // Type/Name of event
    duration: Duration,    // How long the event lasts (seconds)
    dps_modifier: f32,     // Multiplier that affects overall SPS rate
    spc_modifier: f32,
}

struct GameState {
    score: Score,
    buildings: Vec<&'static Building>, // Active/purchased buildings
    events: Vec<Event>,       // Active events
    start_time: Instant,      // When the game was launched
    stats: Statistics,
    menu_type: String,        // Gameplay stats
    owned_buildings: Vec<&'static Building>,
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
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            students_awarded: 0,
            event_type: "".to_string(),
            duration: Duration::from_secs(60),
            dps_modifier: 1.,
            spc_modifier: 1.,
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
    buildings.push(&PSYCHOLOGY);
    buildings.push(&KATHLEENLAMBOURNE);
    buildings.push(&LIVERIS);
    buildings.push(&ADVENG);


    let mut owned_buildings: Vec<&'static Building> = Vec::new();
    
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
    let mut game_state = start_game(buildings, owned_buildings);
    let mut notification_manager = gui::NotificationManager::new();

    let textures = load_textures().await;
    let mut time_el = Instant::now();
    let time_req = Duration::from_secs(1);
    let mut last_event_time = Instant::now();
    let mut current_event: Option<Event> = get_event_from_rand(0, &game_state);

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
        
        gui::gui(&mut notification_manager, &textures, &mut game_state, Some(&font));

        let screen_height = screen_height();
        let screen_width = screen_width();
        let font_size = dynamic_font_size(60.0);

        // Mouse button press function
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < screen_width * 0.7 {
                // ImpClick events added for some of the buy menu rectangles.lement functions for the game.
                game_state.score = clicked(game_state.score, current_event.as_ref());
                sound_effects(String::from("click"), &sounds);
            } else if mouse_x > screen_width * 0.7 {
                // Implement function for buy module.
            }
        }

        gui::build_textdraw(Some(&font), font_size);
        gui::perks_textdraw(Some(&font), font_size);
        gui::stats_textdraw(Some(&font), font_size);


        next_frame().await;
        let duration = time_el.elapsed();
        if (duration >= time_req){
            game_state.score = update_money(game_state.score, current_event.as_ref());
            time_el = Instant::now();
        };

        // Check if ready for an event roll, if ready, roll for an event and add the new event.
        if last_event_time.elapsed() >= Duration::from_secs(60) {
            println!("Rolling for event");

            last_event_time = Instant::now();

            let event = get_event_from_rand(rand::gen_range(0, 30), &game_state);

            if event.is_some() {
                println!("Event Added");
                current_event = event;

                if current_event.as_ref().unwrap().event_type == "AddStudents" {
                    game_state.score.curr_students += current_event.as_ref().unwrap().students_awarded as f64;
                }
            }
        }

        if current_event.as_ref().is_some() {
            if last_event_time.elapsed() >= current_event.as_ref().unwrap().duration {
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
            students_awarded: (-state.score.curr_students as f32 * 0.05) as i32,
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

fn sound_effects(sound: String, sounds: &HashMap<String, PathBuf>) {
    if sound == "click" {
        if let Some(path) = sounds.get("click").cloned() {
            std::thread::spawn(move || {
                music::sound_effect(path, 1);
            });
        }
    }
}

fn start_game(unown: Vec<&'static Building>,
    owned: Vec<&'static Building>) -> GameState {
    let score: Score = Score::init();
    let buildings = unown;
    let owned_buildings: Vec<&Building> = owned;
    let events: Vec<Event> = Vec::new();
    let start_time = Instant::now();
    let stats: Statistics = Statistics::init();
    let menu_type: String = "build".to_string();
    
    GameState {
        score,
        buildings,
        events,
        start_time,
        stats,
        menu_type,
        owned_buildings,
    }
}

async fn load_textures() -> HashMap<String, Texture2D> {
    let buildings = [
        ("Test1", "media/images/BACKGROUND.png"),
        ("Forgan Smith", "media/images/FORGANSMITH.png"),
        ("Goddard Building", "media/images/GODDARD.png"),
        ("Parnell Building", "media/images/PARNELL.png"),
        ("Richards Building", "media/images/RICHARDS.png"),
        ("Steele Building", "media/images/STEELEBUILDING.png"),
        ("EZ Mart", "media/images/EZMART.png"),
        ("Central Library", "media/images/CENTRALLIBRARY.png"),
        // MISSING
        ("Prentice Building", "media/images/RICHARDS.png"),
        ("Learning Innovation Building", "media/images/LEARNINGINNOVATION.png"),
        ("John Hines Building", "media/images/JOHNHINES.png"),
        ("UQ Union and Food Court", "media/images/UNIONFOODCOURT.png"),
        ("McElwain Building", "media/images/MCELWAIN.png"),
        ("Chamberlain Building", "media/images/CHAMBERLAIN.png"),
        ("Art Museum", "media/images/ARTMUSEUM.png"),
        // Missing
        ("Otto Hirschfeld Building", "media/images/RICHARDS.png"),
        ("Molecular BioScience Building", "media/images/MOLECULARBIOSCIENCE.png"),
        ("JD Story Administration Building", "media/images/JDSTORY.png"),
        ("Hartley Teak", "media/images/HARTLEY_TEAK.png"),
        ("Biological Science Library", "media/images/BIO_SCIENCE_LIBRARY.png"),
        ("Brain Institution", "media/images/BRAININSTITUTE.png"),
        ("Center for Water and Environmental Biotechnology", "media/images/WATERANDENVIRO.png"),
        ("Chemistry Building", "media/images/CHEM.png"),
        ("Mansergh Shaw Building", "media/images/MANSERGHSHAW.png"),
        ("Hawken Engineering", "media/images/HAWKEN.png"),
        ("Don Nicklin Building", "media/images/DONNICKLIN.png"),
        ("Bioengineering Institute", "media/images/BIOENG.png"),
        // CANT FIND
        ("Advanced Imaging Centre", "media/images/RICHARDS.png"),
        ("General Purpose South", "media/images/GPSOUTH.png"),
        ("General Purpose North", "media/images/GPNORTH.png"),
        ("UQ Business School", "media/images/UQBUSINESS.png"),
        ("Zelman Cowen Building", "media/images/ZELMANCOWEN.png"),
        ("Building 41", "media/images/BUILDING41.png"),
        ("23, 38, 31A", "media/images/FUNNYNUMBER.png"),
        ("Cumbrae-Stewart Building", "media/images/CUMBRAESTEWART.png"),
        ("O'Connell Building", "media/images/OCONNELL.png"),
        ("Gordon Greenwood Building", "media/images/GORDONGREENWOOD.png"),
        ("UQ Centre", "media/images/UQCENTRE.png"),
        ("Building 33", "media/images/BUILDING33.png"),
        ("Schonell Theatre", "media/images/SCHONELLTHEATRE.png"),
        ("Psychology Building", "media/images/PSYCHOLOGY.png"),
        ("Kathleen Lambourne Building", "media/images/KATHLEENLAMBOURNE.png"),
        ("Advanced Engineering", "media/images/ADVENG.png"),
        ("Andrew N. Liveris Building", "media/images/LIVERIS.png"),
    ];

    let mut textures = HashMap::new();
    // textures.insert("Test1".to_string(), load_texture("media/images/background.png").await.unwrap());
    // // Loading textures of all buildings
    // textures.insert("Forgan Smith".to_string(), load_texture("media/images/FORGANSMITH.png").await.unwrap());
    // textures.insert("Goddard Building".to_string(), load_texture("media/images/FORGANSMITH.png").await.unwrap());
    // textures.insert("Parnell Building".to_string(), load_texture("media/images/parnell.png").await.unwrap());
    // textures.insert("Richards Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("Steele Building".to_string(), load_texture("media/images/STEELEBUILDING.png").await.unwrap());
    // textures.insert("EZ Mart".to_string(), load_texture("media/images/ezmart.png").await.unwrap());
    // textures.insert("Central Library".to_string(), load_texture("media/images/CENTRALLIBRARY.png").await.unwrap());
    // // NOT BEEN ADDED.
    // textures.insert("Prentice Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("Learning Innovation Building".to_string(), load_texture("media/images/LEARNINGINNOVATION.png").await.unwrap());
    // textures.insert("John Hines Building".to_string(), load_texture("media/images/JOHNHINES.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("UQ Union and Food Court".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("McElwain Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Chamberlain Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("Art Museum".to_string(), load_texture("media/images/ARTMUSEUM.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Otto Hirschfeld Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Molecular BioScience Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("JD Story Administration Building".to_string(), load_texture("media/images/JDSTORY.png").await.unwrap());
    // textures.insert("Hartley Teak".to_string(), load_texture("media/images/HARTLEY_TEAK.png").await.unwrap());
    // textures.insert("Biological Science Library".to_string(), load_texture("media/images/BIO_SCIENCE_LIBRARY.png").await.unwrap());
    // textures.insert("Brain Institution".to_string(), load_texture("media/images/BRAININSTITUTE.png").await.unwrap());
    // textures.insert("Center for Water and Environmental Biotechnology".to_string(), load_texture("media/images/WATERANDENVIRO.png").await.unwrap());
    // textures.insert("Chemistry Building".to_string(), load_texture("media/images/CHEM.png").await.unwrap());
    // textures.insert("Mansergh Shaw Building".to_string(), load_texture("media/images/MANSERGHSHAW.png").await.unwrap());
    // textures.insert("Hawken Engineering".to_string(), load_texture("media/images/HAWKEN.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Don Nicklin Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("Bioengineering Institute".to_string(), load_texture("media/images/BIOENG.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Advanced Imaging Centre".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // textures.insert("General Purpose South".to_string(), load_texture("media/images/GPSOUTH.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("General Purpose North".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("UQ Business School".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED 
    // textures.insert("Zelman Cowen Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED 
    // textures.insert("Building 41".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("23, 38, 31A".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Cumbrae-Stewart Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("O'Connell Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Gordon Greenwood Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("UQ Centre" .to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Building 33".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Schonell Theatre".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Psychology Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Kathleen Lambourne Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Advanced Engineering".to_string(), load_texture("media/images/Richards.png").await.unwrap());
    // // NOT BEEN ADDED
    // textures.insert("Andrew N. Liveris Building".to_string(), load_texture("media/images/Richards.png").await.unwrap());

    for (name, path) in &buildings {
        textures.insert(name.to_string(), load_texture(path).await.unwrap());
    }

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
