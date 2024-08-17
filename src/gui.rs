use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::default;
use crate::GameState;

// Score implementations
pub fn score() {}

pub struct Notification {
    text: String,
    timer: f32, // How long the notification should be displayed
}

pub struct NotificationManager {
    notifications: Vec<Notification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    pub fn add_notification(&mut self, text: String, duration: f32) {
        self.notifications.push(Notification {
            text,
            timer: duration,
        });
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update timers and remove expired notifications
        self.notifications.retain_mut(|notification| {
            notification.timer -= delta_time;
            notification.timer > 0.0
        });
    }

    pub fn draw(&self) {}
}

pub fn build_textdraw() {
    let text = "Build";
    let font_size = 40.0;
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = (screen_width() * 0.1);
    let y_pos = (screen_height() * 0.535);
    //let y_pos = (screen_height() * 0.63) + (text_dimensions.height / 2.0);
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.2,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 1.2, // Match the font scale to maintain proportions
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn perks_textdraw() {
    let text = "Perks";
    let font_size = 40.0;
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = (screen_width() * 0.45);
    let y_pos = (screen_height() * 0.535);
    //let y_pos = (screen_height() * 0.63) + (text_dimensions.height / 2.0);
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.2,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 1.2, // Match the font scale to maintain proportions
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn stars_textdraw() {
    let text = "Stars";
    let font_size = 40.0;
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = (screen_width() * 0.8);
    let y_pos = (screen_height() * 0.535);
    //let y_pos = (screen_height() * 0.63) + (text_dimensions.height / 2.0);
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.2,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 1.2, // Match the font scale to maintain proportions
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn gui(notification_manager: &mut NotificationManager, textures: &HashMap<String, Texture2D>, game_state: &GameState) {
    let screen_height = screen_height();
    let screen_width = screen_width();
    // Define the dimensions and positions of the rectangles
    let rects = [
        Rect::new(0.5 * screen_width, 0.0, 0.66 * screen_width, 0.09 * screen_height),  // Stats
        Rect::new((-0.33  + 1.0) * screen_width * 0.5, 0.0, 0.66 * screen_width, 0.09 * screen_height), // Build
        Rect::new((0.34 + 1.0) * screen_width * 0.5, 0.0, 0.66 * screen_width, 0.09 * screen_height), // Perks
        Rect::new(0.0, screen_height * 0.18, screen_width * 2.0, screen_height * 0.16),  // Other red rectangles
        Rect::new(0.0, screen_height * 0.36, screen_width * 2.0, screen_height * 0.16),
        Rect::new(0.0, screen_height * 0.54, screen_width * 2.0, screen_height * 0.16),
        Rect::new(0.0, screen_height * 0.72, screen_width * 2.0, screen_height * 0.16),
        Rect::new(0.0, screen_height * 0.9, screen_width * 2.0, screen_height * 0.16),
    ];

    // Handle click events
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position = mouse_position();
        
        for (index, rect) in rects.iter().enumerate() {
            if rect.contains(mouse_position.into()) {
                // Trigger the corresponding event based on index
                match index {
                    0 => {
                        println!("CLICKED 0");
                    },
                    1 => {
                        println!("CLICKED 1");
                    },
                    2 => {
                        println!("CLICKED 2");
                    },
                    3 => {
                        println!("CLICKED 3");
                    },
                    4 => {
                        println!("CLICKED 4");
                    },
                    5 => {
                        println!("CLICKED 5");
                    },
                    6 => {
                        println!("CLICKED 6");
                    },
                    7 => {
                        println!("CLICKED 7");
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

    let buy_frame_width = (screen_width * 0.7) / 2 as f32;

    let stat_frame = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0, 1.0),
        viewport: Some((
            (buy_frame_width - 800.0) as i32,
            (screen_height * 0.85) as i32,
            (screen_width * 0.7) as i32,
            (screen_height * 0.3) as i32,
        )),

        ..Default::default()
    };

    //Scale the game map to fit a 1:1 aspect ratio and draw the game map
    let game_window_dimensions = ((screen_width * 0.7) as i32, screen_height as i32);

    let map_size = min(game_window_dimensions.0, game_window_dimensions.1);

    let map_x_pos = max(0, (game_window_dimensions.0 - map_size) / 2) as f32;
    let map_y_pos = max(0, (game_window_dimensions.1 - map_size) / 2) as f32;

    draw_texture_ex(textures.get("Test1").unwrap(), map_x_pos, map_y_pos, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(map_size as f32, map_size as f32)),
        ..Default::default()
    });

    // Draw the buy frame
    set_camera(&buy_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.3, screen_height, LIGHTGRAY);

    // Draw smaller rectangles inside the buy frame
    draw_rectangle(-1.0, 0.0, 2.0, 0.1, BLACK); // Top rectangle, holds Build | Perks | Stats
    draw_rectangle(-1.0, 0.0, 0.66, 0.09, BLUE); // Stats
    draw_rectangle(-0.33, 0.0, 0.66, 0.09, GREEN); // Build
    draw_rectangle(0.34, 0.0, 0.66, 0.09, ORANGE); // Perks

    draw_rectangle(-1.0, 0.18, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.36, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.54, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.72, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.9, 2.0, 0.16, RED);

    let widget_width = ((screen_width * 0.64) /2.) as f32;

    widgets::Window::new(hash!(), vec2(widget_width, 0.), vec2(550., 50.))
        .label("Statistics")
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(Vec2::new(10., 10.), "Total Students:");
            ui.label(Vec2::new(130., 10.), &game_state.score.curr_students.to_string());
            ui.label(Vec2::new(250., 10.), "Currency $: ");
            ui.label(Vec2::new(370., 10.), &game_state.score.dollars.to_string());
        });
    // Reset to default camera
    set_default_camera();
    build_textdraw();
    perks_textdraw();
    stars_textdraw();

    notification_manager.update(get_frame_time());
    notification_manager.draw();
}
