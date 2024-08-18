use macroquad::prelude::*;
use macroquad::ui::widgets::Window;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::default;
use std::thread::sleep;
use crate::music;

use crate::{Event, GameState};

static light_blue:macroquad::color::Color = Color::new(0.0078, 0.4392, 0.9098, 0.77); // Normalized values
static middle_blue:macroquad::color::Color = Color::new(0.0078, 0.4392, 0.9098, 0.86); // Normalized values
static dark_blue:macroquad::color::Color = Color::new(0.0078, 0.4392, 0.9098, 1.0); // Normalized values

pub fn build_textdraw(font: Option<&Font>, font_size: u16) {
    let text = "Build";
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = screen_width() * 0.2;
    let y_pos = screen_height() * 0.535;
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size,
            font_scale: 0.7,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 3.0, // Match the font scale to maintain proportions
            font: font,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn perks_textdraw(font: Option<&Font>, font_size: u16) {
    let text = "Perks";
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = screen_width() * 0.7;
    let y_pos = screen_height() * 0.535;
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size,
            font_scale: 0.7,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 3.0, // Match the font scale to maintain proportions
            font: font,
            color: BLACK,
            ..Default::default()
        },
    );
}

// pub fn stats_textdraw(font: Option<&Font>, font_size: u16) {
//     let text = "Stats";
//     let x_pos = screen_width() * 0.8;
//     let y_pos = screen_height() * 0.535;
//     draw_text_ex(
//         text,
//         x_pos,
//         y_pos,
//         TextParams {
//             font_size: font_size,
//             font_scale: 0.7,        // Slight horizontal scale to make the text wider
//             font_scale_aspect: 3.0, // Match the font scale to maintain proportions
//             color: BLACK,
//             font: font,
//             ..Default::default()
//         },
//     );
// }

pub fn buymenu_font(font: Option<&Font>, font_size: u16, text: String, box_number: usize) {
    let box_coords: [[f32; 2]; 5] = [
        [0.35 , 0.575],
        [0.35 , 0.66],
        [0.35 , 0.745],
        [0.35 , 0.83],
        [0.35 , 0.915],
    ];
    // 27 char limit
    // Define the dimensions and positions of the rectangles
    let x_pos = screen_width() * box_coords[box_number][0];
    // bigger for further
    let y_pos = screen_height() * box_coords[box_number][1];
    draw_text_ex(
        &text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size,
            font_scale: 0.7,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 3.0, // Match the font scale to maintain proportions
            color: BLACK,
            font: font,
            ..Default::default()
        },
    );
}

pub fn buymenu_price(font: Option<&Font>, font_size: u16, price: i64, box_number: usize) {
    let box_coords: [[f32; 2]; 5] = [
        [0.6, 0.575],
        [0.6, 0.66],
        [0.6, 0.745],
        [0.6, 0.83],
        [0.6, 0.915],
    ];
    // Define the dimensions and positions of the rectangles
    let x_pos = screen_width() * box_coords[box_number][0];
    // bigger for further
    let y_pos = screen_height() * box_coords[box_number][1];
    let price_str = format!("${}", price);
    draw_text_ex(
        &price_str,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size,
            font_scale: 0.7,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 3.0, // Match the font scale to maintain proportions
            color: BLACK,
            font: font,
            ..Default::default()
        },
    );
}

pub fn buymenu_description(font: Option<&Font>, font_size: u16, text: String, box_number: usize) {
    // Define the coordinates for the text box
    let box_coords: [[f32; 2]; 5] = [
        [0.35 , 0.59],
        [0.35 , 0.675],
        [0.35 , 0.76],
        [0.35 , 0.845],
        [0.35 , 0.93],
    ];
    
    // Set the maximum number of characters per line
    // 148 description limit
    let char_limit = 40;
    
    // Split the text into lines based on the character limit
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        // Check if adding the next word exceeds the character limit
        if current_line.len() + word.len() + 1 > char_limit {
            lines.push(current_line);
            current_line = String::new();
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    
    // Push the last line if it's not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    // Draw each line of text at the appropriate position
    let x_pos = screen_width() * box_coords[box_number][0];
    let mut y_pos = screen_height() * box_coords[box_number][1];

    for line in lines {
        draw_text_ex(
            &line,
            x_pos,
            y_pos,
            TextParams {
                font_size: font_size,
                font_scale: 0.3,        // Slight horizontal scale to make the text wider
                font_scale_aspect: 5.0, // Match the font scale to maintain proportions
                color: BLACK,
                font: font,
                ..Default::default()
            },
        );
        // Adjust the y-position for the next line
        y_pos += font_size as f32 * 0.25; // Adjust this factor to control line spacing
    }
}


fn buy_building(game_state: &mut GameState, index: i32){
    if !game_state.buildings.is_empty() {
        let sounds = crate::setup_sounds();

        if (index == 0) {
            
            if let Some(building) = game_state.buildings.get(0) {
                let building_name = building.name;
                let building_price = building.price;
                println!("{} {}",building_name, building_price);
                if (game_state.score.dollars>= building.price as f64){
            // Remove the building from index 0 and append it directly to owned_buildings
                    let building = game_state.buildings.remove(0);
                    game_state.owned_buildings.push(building);
                    game_state.score.curr_students += building.students;
                    game_state.score.dollars -= building.price as f64;
                    if let Some(path) = sounds.get("cash").cloned() {
                        std::thread::spawn(move || {
                            music::sound_effect(path, 2);
                        });
                    }
                }
            }
            
            
        }
        if (index == 1) {
            
            if let Some(building) = game_state.buildings.get(1) {
                let building_name = building.name;
                let building_price = building.price;
                println!("{} {}",building_name, building_price);
                if (game_state.score.dollars>= building.price as f64){
            // Remove the building from index 0 and append it directly to owned_buildings
                    let building = game_state.buildings.remove(1);
                    game_state.owned_buildings.push(building);
                    game_state.score.curr_students += building.students;
                    game_state.score.dollars -= building.price as f64;
                    if let Some(path) = sounds.get("cash").cloned() {
                        std::thread::spawn(move || {
                            music::sound_effect(path, 2);
                        });
                    }
                }
            }
            
        }
        if (index == 2) {
            
            if let Some(building) = game_state.buildings.get(2) {
                let building_name = building.name;
                let building_price = building.price;
                println!("{} {}",building_name, building_price);
                if (game_state.score.dollars>= building.price as f64){
            // Remove the building from index 0 and append it directly to owned_buildings
                    let building = game_state.buildings.remove(2);
                    game_state.owned_buildings.push(building);
                    game_state.score.curr_students += building.students;
                    game_state.score.dollars -= building.price as f64;
                    if let Some(path) = sounds.get("cash").cloned() {
                        std::thread::spawn(move || {
                            music::sound_effect(path, 2);
                        });
                    }
                }
            }
            
        }
        if (index == 3) {
            
            if let Some(building) = game_state.buildings.get(3) {
                let building_name = building.name;
                let building_price = building.price;
                println!("{} {}",building_name, building_price);
                if (game_state.score.dollars>= building.price as f64){
            // Remove the building from index 0 and append it directly to owned_buildings
                    let building = game_state.buildings.remove(3);
                    game_state.owned_buildings.push(building);
                    game_state.score.curr_students += building.students;
                    game_state.score.dollars -= building.price as f64;
                    if let Some(path) = sounds.get("cash").cloned() {
                        std::thread::spawn(move || {
                            music::sound_effect(path, 2);
                        });
                    }
                }
            }
            
        }
        if (index == 4) {
            
            if let Some(building) = game_state.buildings.get(4) {
                let building_name = building.name;
                let building_price = building.price;
                println!("{} {}",building_name, building_price);
                if (game_state.score.dollars>= building.price as f64){
            // Remove the building from index 0 and append it directly to owned_buildings
                    let building = game_state.buildings.remove(4);
                    game_state.owned_buildings.push(building);
                    game_state.score.curr_students += building.students;
                    game_state.score.dollars -= building.price as f64;
                    if let Some(path) = sounds.get("cash").cloned() {
                        std::thread::spawn(move || {
                            music::sound_effect(path, 2);
                        });
                    }
                }
            }
            
        }
        
    }
    
    // for building in &game_state.buildings {
    //     let building_name = building.name;
    //     println!("{}",building_name);
    // }
}
pub fn gui(textures: &HashMap<String, Texture2D>, game_state: &mut GameState, font: Option<&Font>) {
   
    let screen_height = screen_height();
    let screen_width = screen_width();
    let buy_frame_width = (screen_width * 0.7) / 2 as f32;
    // Define the dimensions and positions of the rectangles
    let rects = [
        Rect::new(screen_width - buy_frame_width, 0.0, 0.5 * buy_frame_width, 0.09 * screen_height), 
        Rect::new(screen_width - buy_frame_width + (buy_frame_width / 2.0), 0.0, 0.5 * buy_frame_width, 0.09 * screen_height),
        Rect::new(screen_width - buy_frame_width, screen_height * 0.1, buy_frame_width, screen_height * 0.16),
        Rect::new(screen_width - buy_frame_width, screen_height * 0.27, buy_frame_width, screen_height * 0.16),
        Rect::new(screen_width - buy_frame_width, screen_height * 0.44, buy_frame_width, screen_height * 0.16),
        Rect::new(screen_width - buy_frame_width, screen_height * 0.61, buy_frame_width, screen_height * 0.16),
        Rect::new(screen_width - buy_frame_width, screen_height * 0.78, buy_frame_width, screen_height * 0.16),
    ];
    //Positioning variables for currency widget
    
    // Handle click events
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position = mouse_position();
        
        for (index, rect) in rects.iter().enumerate() {
            if rect.contains(mouse_position.into()) {
                // Trigger the corresponding event based on index
                match index {
                    0 => { // Build Button
                        println!("CLICKED 0");
                        game_state.menu_type = "build".to_string();
                        println!("{}",game_state.menu_type);
                    },
                    1 => { // Perks Button
                        println!("CLICKED 1");
                        game_state.menu_type = "perks".to_string();
                        println!("{}",game_state.menu_type);
                    },
                    2 => {
                        println!("CLICKED 3");
                        buy_building(game_state,0);
                    },
                    3 => {
                        println!("CLICKED 4");
                        buy_building(game_state,1);
                    },
                    4 => {
                        println!("CLICKED 5");
                        buy_building(game_state,2);
                    },
                    5 => {
                        println!("CLICKED 6");
                        buy_building(game_state,3);
                    },
                    6 => {
                        println!("CLICKED 7");
                        buy_building(game_state,4);
                    },
                    _ => {}
                }
            }
        }
    }
    
    // // Default game frame
    // // viewport, x = 0, y = 0, width, height.
    // let game_frame = Camera2D {
    //     target: vec2(0.0, 0.0),
    //     zoom: vec2(1.0, 1.0),
    //     viewport: Some((0, 0, (screen_width * 0.7) as i32, screen_height as i32 * 2)),
    //     ..Default::default()
    // };

    // Frame for buying upgrades, perks, etc.
    // viewport, x = 540, y = 0, width, height.
    let buy_frame = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0, 1.0),
        viewport: Some((
            (screen_width * 0.7) as i32,
            0,
            (screen_width * 0.3) as i32,
            screen_height as i32 * 2,
        )),
        ..Default::default()
    };

    // let stat_frame = Camera2D {
    //     target: vec2(0.0, 0.0),
    //     zoom: vec2(1.0, 1.0),
    //     viewport: Some((
    //         (buy_frame_width - 800.0) as i32,
    //         (screen_height * 0.85) as i32,
    //         (screen_width * 0.7) as i32,
    //         (screen_height * 0.3) as i32,
    //     )),

    //     ..Default::default()
    // };

    
    //Scale the game map to fit a 1:1 aspect ratio and draw the game map
    let game_window_dimensions = ((screen_width * 0.7) as i32, screen_height as i32);

    let background_texture = textures.get("Background").unwrap();
    let foreground_texture = textures.get("Foreground").unwrap();
    let ratio = background_texture.width() / background_texture.height();

    let map_size_x = min(game_window_dimensions.0, (game_window_dimensions.1 as f32 * ratio) as i32);
    let map_size_y = map_size_x as f32 / ratio;

    let map_x_pos = max(0, (game_window_dimensions.0 - map_size_x) / 2) as f32;
    let map_y_pos = max(0, (game_window_dimensions.1 - map_size_y as i32) / 2) as f32;

    draw_texture_ex(background_texture, map_x_pos, map_y_pos, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(map_size_x as f32, map_size_y as f32)),
        ..Default::default()
    });



    let widget_width = min(500, game_window_dimensions.0) as f32;
    let window_position_x = (game_window_dimensions.0 as f32 - widget_width) / 2.;
    let students_pos = 10.;
    let currency_pos = if widget_width / 2. < 150. {students_pos} else {widget_width / 2.};
    let currency_height = if currency_pos == 10. {34} else {10} as f32;
    let widget_height = if currency_pos == 10. {65} else {50} as f32;
    // root_ui().window(1, vec2(window_position_x, 0.), vec2(widget_width, widget_height), |ui| {
    //     ui.label(Vec2::new(10., 10.), "Total Students:");
    //     ui.label(Vec2::new(130., 10.), &(game_state.score.curr_students as i32).to_string());
    //     ui.label(Vec2::new(currency_pos, currency_height), "Currency $: ");
    //     ui.label(Vec2::new((currency_pos) + 95., currency_height - 2.), &(game_state.score.dollars as i32).to_string());
    // });
    // Iterating through owned buildings.
    for building in &game_state.owned_buildings {
        let building_name = building.name;
        // println!("{}",building_name);
        if let Some(texture) = textures.get(building_name) {
            draw_texture_ex(
                texture,
                map_x_pos,
                map_y_pos,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(map_size_x as f32, map_size_y as f32)),
                    ..Default::default()
                }
            );
        } else {
            println!("Texture for building '{}' not found!", building_name);
        }
    }

    draw_texture_ex(foreground_texture, map_x_pos, map_y_pos, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(map_size_x as f32, map_size_y as f32)),
        ..Default::default()
    });

    // Draw the buy frame
    set_camera(&buy_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.3, screen_height, BLACK);

    // Draw smaller rectangles inside the buy frame
    draw_rectangle(-1.0, 0.0, 2.0, 0.1, BLACK); // Top rectangle, holds Build | Perks | Stats
    draw_rectangle(-1.0, 0.0, 1.0, 0.09, middle_blue); 
    draw_rectangle(0.0, 0.0, 1.0, 0.09, dark_blue); 

    draw_rectangle(-1.0, 0.1, 2.0, 0.16, LIGHTGRAY);
    draw_rectangle(-1.0, 0.27, 2.0, 0.16, LIGHTGRAY);
    draw_rectangle(-1.0, 0.44, 2.0, 0.16, LIGHTGRAY);
    draw_rectangle(-1.0, 0.61, 2.0, 0.16, LIGHTGRAY);
    draw_rectangle(-1.0, 0.78, 2.0, 0.16, LIGHTGRAY);
  
    //Positioning variables for currency widget
    

    //If screen width has changed, move the window to new position
    root_ui().move_window(1, Vec2::new(window_position_x, 0.));

    // Reset to default camera
    set_default_camera();
    
    //Draw currency widget
    root_ui().window(1, vec2(window_position_x, 0.), vec2(widget_width, widget_height), |ui| {
        ui.label(Vec2::new(10., 10.), "Total Students:");
        ui.label(Vec2::new(130., 10.), &(game_state.score.curr_students as i32).to_string());
        ui.label(Vec2::new(currency_pos, currency_height), "Currency $: ");
        ui.label(Vec2::new((currency_pos) + 95., currency_height - 2.), &(game_state.score.dollars as i32).to_string());
    });


}

pub fn draw_event_gui(event: &Event) -> bool {
    let screen_height = screen_height();
    let screen_width = screen_width();
    let mut outcome = true;

    let mut heading = "".to_string();
    let mut description = "".to_string();
    let mut duration = "".to_string();
    let mut effect = "".to_string();

    if (*event).event_type == "CashProduction" {
        if (*event).dps_modifier == 2. {
            heading = "\"Inflation\"".to_string();
            description = "Due to an increase in student fees, UQ's cash production per student is doubled".to_string();
            effect = "Student cash producion: 2x".to_string();
            duration.push_str("Duration: ");
            duration.push_str(&(*event).duration.as_secs().to_string());
            duration.push_str("s");
        } else if (*event).dps_modifier == 0.5 {
            heading = "International politics".to_string();
            description = "Changes in international immigration policy have reduced UQ's income from international students".to_string();
            effect = "Student cash producion: 0.5x".to_string();
            duration.push_str("Duration: ");
            duration.push_str(&(*event).duration.as_secs().to_string());
            duration.push_str("s");
        }
    } else if (*event).event_type == "AddStudents" {
        if (*event).students_awarded >= 0 {
            heading = "Positive Media Attention".to_string();
            description = "The ABC has written a glowing piece on UQ. You gain 5% to your total students".to_string();
            effect = "Students gained: ".to_string();
            effect.push_str(&(*event).students_awarded.to_string());
        } else if (*event).students_awarded >= 0 {
            heading = "Negative Media Attention".to_string();
            description = "UQ has been involved in a major scandal! You lose 5% of your total students".to_string();
            effect = "Students gained: ".to_string();
            effect.push_str(&(*event).students_awarded.to_string());
        }
    } else if (*event).event_type == "StudentsPerClick" {
        if (*event).spc_modifier == 2. {
            heading = "University Ranking Increased".to_string();
            description = "Global university rankings have been released. UQ's rank inceased from last year. Student gain per click is doubled".to_string();
            effect = "Student gain per click: 2x".to_string();
            duration.push_str("Duration: ");
            duration.push_str(&(*event).duration.as_secs().to_string());
            duration.push_str("s");
        } else if (*event).spc_modifier == 0.5 {
            heading = "University Ranking Decreased".to_string();
            description = "Global university rankings have been released. UQ's rank decreased from last year. Student gain per click is halved".to_string();
            effect = "Student gain per click: 0.5x".to_string();
            duration.push_str("Duration: ");
            duration.push_str(&(*event).duration.as_secs().to_string());
            duration.push_str("s");
        }
    }

    let len = description.len();
    let widget_width = 500.;
    let widget_height = 400.;

    let pixels_per_char = 7.5;
    let num_rows = ((len as f64 * pixels_per_char) / widget_width) as i32 + 1;

    let chars_per_line: i32 = (widget_width / pixels_per_char) as i32;


    root_ui().window(2, vec2((screen_width / 2.) - 250., (screen_height / 2.) - 200.), vec2(500., 400.), |ui| {
        ui.label(Vec2::new(10., 10.), &heading);

        let mut shift = 20.;
        for n in 0..num_rows {
            ui.label(Vec2::new(10., 10. + shift), &description[(n * chars_per_line) as usize..min(((n + 1) * (chars_per_line)) as usize, len)]);
            shift += 20.;
        }
        
        ui.label(Vec2::new(10., 10. + shift), &effect);
        ui.label(Vec2::new(10., 30. + shift), &duration);
        if ui.button(Vec2::new(225., 300.), "Close") {
            outcome = false;
        }
    });

    return outcome;

    //If screen width has changed, move the window to new position
    // root_ui().move_window(1, Vec2::new(window_position_x, 0.));
} 
