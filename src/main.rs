use macroquad::prelude::*;

mod game;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static WIDTH: u16 = 640;
pub static HEIGHT: u16 = 480;

#[macroquad::main(settings)] // Macroquad creates a window
async fn main() {

    let mut game = game::Game::new(); // Create an instance to hold game variables and structures

    game.load(); // Load stuff

    loop { // runs at monitor refresh rate (usually 60 times per second)
        game.update(get_frame_time()); // Update the game state (with delta (frame) time so physics and such can run at a constant speed no matter what the framerate is)
        clear_background(GRAY);
        game.render(); // render the stuff on screen
        next_frame().await; // wait for the next frame before looping
    }

}

fn settings() -> Conf { // Window settings
    Conf {
        window_title: String::from(NAME),
        window_width: WIDTH as _,
        window_height: HEIGHT as _,
        ..Default::default()     
    }
}