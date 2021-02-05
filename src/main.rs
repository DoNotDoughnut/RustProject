use configuration::Configuration;
use macroquad::prelude::*;

mod game;
mod input;
mod configuration;
mod world;

pub static NAME: &str = env!("CARGO_PKG_NAME"); // name of project in Cargo.toml used as a variable in code
pub static SCALE: u16 = 3; // pixel scaling (pixels drawn on window are 2x normal pixel size)
pub static WIDTH: u16 = 240; // view width
pub static HEIGHT: u16 = 160; // view height

#[macroquad::main(settings)] // Macroquad creates a window
async fn main() {

    info!("Starting {}", NAME);

    let mut game = game::Game::new(); // Create an instance to hold game variables and structures

    game.load().await; // Load stuff

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32)); // Create a camera to view the screen with
    set_camera(camera); // activate the camera

    loop { // runs at monitor refresh rate (usually 60 times per second)
        
        game.update(get_frame_time()); // Update the game state (with delta (frame) time so physics and such can run at a constant speed no matter what the framerate is)
        clear_background(GRAY);
        game.render(); // render the stuff on screen
        next_frame().await; // wait for the next frame before looping
    }

}

fn settings() -> Conf { // Window settings
    let configuration = Configuration::default();
    println!("{}", nanoserde::SerJson::serialize_json(&configuration)); // can save and load this from a JSON file, this is an example for now
    Conf {
        window_title: configuration.name,
        window_width: configuration.window_width as _,
        window_height: configuration.window_height as _,
        ..Default::default()     
    }
}

pub trait Entity {

    fn update(&mut self, delta: f32);

    fn render(&self);

}