use macroquad::prelude::get_fps;
use macroquad::prelude::info;

use crate::Entity;
use crate::menu::MainMenu;
use crate::world::World;

pub struct Game {

    game_state: GameState,
    main_menu: MainMenu,
    world: World,

}

pub static mut GAME_STATE: GameState = GameState::MainMenu;

impl Game {

    pub fn new() -> Self {
        Self {
            game_state: GameState::MainMenu,
            main_menu: MainMenu::new(),
            world: World::new(),
        }
    }

    pub async fn load(&mut self) {
        self.world.load_level().await;
    }
    
    pub fn update(&mut self, delta: f32) { // delta * frame rate = 1
        unsafe {
            if GAME_STATE != self.game_state {
                self.quit_state();
                self.game_state = GAME_STATE;
            }
        }
        match &self.game_state {
            GameState::MainMenu => {
                self.main_menu.update(delta);
            }
            GameState::World => {
                self.world.update(delta);
                if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
                    info!("Frame Time: {}, FPS: {}", delta, get_fps());
                }
            }
        }
        
    }
    
    pub fn render(&self) {
        match &self.game_state {
            GameState::MainMenu => {
                self.main_menu.render();
            }
            GameState::World => {
                self.world.render();
            }
        }
    }

    fn quit_state(&mut self) {
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