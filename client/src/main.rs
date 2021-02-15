use macroquad::prelude::*;

mod game;
mod character;
mod io;
mod graphics;
mod gui;

// mod net;

mod menu;
mod world;

pub static NAME: &str = "Brawlstars Clone"; // name of project
pub static SCALE: f32 = 4.0; // pixel scaling (pixels drawn on window are 4x normal pixel size)
pub static WIDTH: u16 = 320; // view width
pub static HEIGHT: u16 = WIDTH * 9 / 16; // view height

pub static DESKTOP: bool = cfg!(target_os = "windows") || cfg!(target_os = "macos") || cfg!(target_os = "linux");

static mut QUIT: bool = false; // Creates a boolean that, when false, allows the program to end

#[macroquad::main(settings)] // Macroquad creates a window
async fn main() {

    info!("Starting client for {}", NAME);

    //let configuration = crate::configuration::Configuration::load_or_default().await;

    let mut game = crate::game::Game::new(); // Create an instance to hold game variables and structures

    game.load().await; // Load stuff

    // let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32)); // Create a camera to view the screen with
    // set_camera(camera); // activate the camera

    loop { // runs at monitor refresh rate (usually 60 times per second)
        // socket.manual_poll(std::time::Instant::now());
        
        game.update(macroquad::prelude::get_frame_time()); // Update the game state (with delta (frame) time so physics and such can run at a constant speed no matter what the framerate is)
        macroquad::prelude::clear_background(macroquad::prelude::BROWN);
        game.render(); // render the stuff on screen
        if unsafe{crate::QUIT} {
            break;
        }
        macroquad::prelude::next_frame().await; // wait for the next frame before looping

    }

    game.quit(); // Run the quit() method for stuff that needs to do things when the game closes   

}

fn settings() -> Conf { // Window settings
    Conf {
        window_title: NAME.to_owned(),
        window_width: WIDTH as i32 * SCALE as i32,
        window_height: HEIGHT as i32 * SCALE as i32,
        sample_count: 4,
        ..Default::default()     
    }
}

pub fn quit() { // Function to run to queue the close sequence of the app
    unsafe {
        QUIT = true;
    }
}

pub trait Entity {

    fn update(&mut self, delta: f32);

    fn render(&self);

}