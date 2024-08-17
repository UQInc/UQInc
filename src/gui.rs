use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

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

pub fn gui(notification_manager: &mut NotificationManager) {
    let screen_height = screen_height();
    let screen_width = screen_width();
    // Define the dimensions and positions of the rectangles
    let rects = [
        Rect::new(0.0, 0.0, screen_width * 0.66, screen_height * 0.09),  // Stats
        Rect::new(screen_width * 0.33, 0.0, screen_width * 0.66, screen_height * 0.09),  // Build
        Rect::new(screen_width * 0.67, 0.0, screen_width * 0.66, screen_height * 0.09),  // Perks
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
    // Default game frame
    // viewport, x = 0, y = 0, width, height.
    let game_frame = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0, 1.0),
        viewport: Some((0, 0, (screen_width * 0.7) as i32, screen_height as i32 * 2)),
        ..Default::default()
    };

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
            (buy_frame_width * -0.5) as i32,
            (screen_height * 0.85) as i32,
            (screen_width * 0.7) as i32,
            (screen_height * 0.3) as i32,
        )),

        ..Default::default()
    };

    // Draw the game frame
    set_camera(&game_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.7, screen_height, GRAY);

    // Draw the buy frame
    set_camera(&buy_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.3, screen_height, LIGHTGRAY);
    // Draw smaller rectangles inside the buy frame
    draw_rectangle(-1.0, 0.0, 2.0, 0.1, GREEN); // Top rectangle, holds Build | Perks | Stats
    draw_rectangle(-1.0, 0.0, 0.66, 0.09, BLUE); // Stats
    draw_rectangle(-0.33, 0.0, 0.66, 0.09, BLACK); // Build
    draw_rectangle(0.34, 0.0, 0.66, 0.09, YELLOW); // Perks

    draw_rectangle(-1.0, 0.18, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.36, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.54, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.72, 2.0, 0.16, RED);
    draw_rectangle(-1.0, 0.9, 2.0, 0.16, RED);

    set_camera(&stat_frame);
    draw_rectangle(0.0, 0.0, screen_width * 0.3, screen_height * 0.2, WHITE);

    // Reset to default camera
    set_default_camera();

    notification_manager.update(get_frame_time());
    notification_manager.draw();
}
