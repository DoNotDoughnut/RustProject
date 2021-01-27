use macroquad::prelude::get_fps;
use macroquad::prelude::info;

use crate::player::Player;

pub struct Game {

    player: Player,

}

impl Game {

    pub fn new() -> Self {
        Self {
            player: Player::default(),
        }
    }

    pub async fn load(&mut self) {

    }
    
    pub fn update(&mut self, delta: f32) { // delta * frame rate = 1
        self.player.update(delta);
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
            info!("Frame Time: {}, FPS: {}", delta, get_fps());
        }
    }
    
    pub fn render(&self) {
        self.player.render();
    }
    
    // pub fn quit(&mut self) {}

}

