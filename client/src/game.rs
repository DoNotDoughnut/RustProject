use macroquad::prelude::get_fps;
use macroquad::prelude::info;

use crate::Entity;
use crate::menu::MainMenu;
use crate::world::World;

pub struct Game {

    game_state: GameState,
    main_menu: MainMenu,
    world: World, // World is deprecated, will replace with newer code so there is little comments

}

static mut GAME_STATE: GameState = GameState::MainMenu; // Set the global gamestate variable that can be changed

impl Game {

    pub fn new() -> Self { // Create a new instance of the game
        Self {
            game_state: GameState::MainMenu,
            main_menu: MainMenu::new(),
            world: World::new(),
        }
    }

    pub async fn load(&mut self) {
        self.world.load_level().await; // Load the world (probably deprecated)
    }
    
    pub fn update(&mut self, delta: f32) { // delta * frame rate = 1
        unsafe {
            if GAME_STATE != self.game_state { // Change the game state if it is different to the stored one
                self.quit_state(); // Run the code that is supposed to run when a game state stops
                self.game_state = GAME_STATE; // Update the stored game state
            }
        }
        match &self.game_state {
            GameState::MainMenu => {
                self.main_menu.update(delta);
            }
            GameState::World => {
                self.world.update(delta);
                if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) { 
                    info!("Frame Time: {}, FPS: {}", delta, get_fps()); // Show frame time and FPS when F1 is clicked
                }
            }
        }
        
    }
    
    pub fn render(&self) { // Render the game
        match &self.game_state { // Choose the correct game state to render
            GameState::MainMenu => {
                self.main_menu.render();
            }
            GameState::World => {
                self.world.render();
            }
        }
    }

    fn quit_state(&mut self) { // Unimplemented
        match &self.game_state {
            GameState::MainMenu => {

            }
            GameState::World => {
                
            }
        }
    }
    
    pub fn quit(&mut self) {}

}

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {

    MainMenu,
    World,

}

pub fn change_game_state(game_state: GameState) {
    unsafe {
        GAME_STATE = game_state; // Change the game state to the one provided
    }
}