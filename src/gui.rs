use macroquad::prelude::*;

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
    draw_rectangle(-1.0, 0.0, 2.0, 0.2, GREEN);
    draw_rectangle(-1.0, 0.21, 2.0, 0.2, RED);
    draw_rectangle(-1.0, 0.42, 2.0, 0.2, RED);
    draw_rectangle(-1.0, 0.63, 2.0, 0.2, RED);
    draw_rectangle(-1.0, 0.84, 2.0, 0.2, RED);

    // Reset to default camera for text drawing
    set_default_camera();

    // Slightly increase text width by scaling it horizontally
    let text = "Hello, Macroquad!";
    let font_size = 40.0;
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x_pos = (screen_width * 0.7) + ((screen_width * 0.3 - text_dimensions.width * 1.2) / 2.0);
    let y_pos = (screen_height * 0.63) + (text_dimensions.height / 2.0);
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.2,        // Slight horizontal scale to make the text wider
            font_scale_aspect: 1.0, // Keep the vertical scaling unchanged
            color: BLACK,
            ..Default::default()
        },
    );

    set_camera(&stat_frame);
    draw_rectangle(0.0, 0.0, screen_width * 0.3, screen_height * 0.2, WHITE);

    // Reset to default camera
    set_default_camera();

    notification_manager.update(get_frame_time());
    notification_manager.draw();
}
