use input::ControlState;
use piston_window::EventLoop;
use piston_window::{UpdateEvent, RenderEvent, PressEvent, ReleaseEvent};
use enum_iterator::IntoEnumIterator;

mod game;
mod input;

fn main() {
    
    // Build window

    let mut window: piston_window::PistonWindow = 
        piston_window::WindowSettings::new("Window", [1280, 720]) // Create window with a title and size
        .vsync(true) // Vsync caps framerate to monitor refresh rate
        .build().expect("Could not build piston window!"); // Attempt to create window (Should almost always work but catch and error if not)

    window.set_ups(30); // game updates 30 times per second

    // Create game instance

    let mut game = game::Game::new();

    // Load stuff

    game.load();

    // Run game loop

    let controls = input::Control::into_enum_iter();

    while let Some(e) = window.next() {

        // Handle input changes and update

        if let Some(_) = e.update_args() {
            game.input();
            for i in controls {
                if game.input.controls[i] == ControlState::Pressed { // if key = pressed,
                    game.input.controls[i] = ControlState::Held; // key = held down
                }
            }
            game.update();
        }

        // Render stuff to screen

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |mut ctx, g, _device| {
                piston_window::clear([0.0, 0.0, 0.0, 1.0], g); // clear the last frame from the screen
                game.render(&mut ctx, g); // render the new frame
            });
        }

        // Handle button presses

        if let Some(ref button) = e.press_args() {
            if let Some(i) = game.input.buttonmap.get(button) {
                game.input.controls[*i] = ControlState::Pressed;
            }
        }

        if let Some(ref button) = e.release_args() {
            if let Some(i) = game.input.buttonmap.get(button) {
                game.input.controls[*i] = ControlState::Up;
            }
        }

    }

    // Run a function when quitting game

    game.quit();

}