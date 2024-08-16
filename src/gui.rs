use macroquad::prelude::*;

// Score implementations
pub fn score {

}


pub fn gui() {
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
        viewport: Some(((screen_width * 0.7) as i32, 0, (screen_width * 0.3) as i32, screen_height as i32 * 2)),
        ..Default::default()
    };

    // Draw the game frame
    set_camera(&game_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.7, screen_height, GRAY);

    // Draw the buy frame
    set_camera(&buy_frame);
    draw_rectangle(-1.0, 0.0, screen_width * 0.3, screen_height, LIGHTGRAY);


    // Reset to default camera
    set_default_camera();
}

