use macroquad::prelude::get_fps;
use macroquad::prelude::info;

use crate::Entity;
use crate::world::World;

pub struct Game {

    world: World,

}

impl Game {

    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub async fn load(&mut self) {
        self.world.load_level().await;
    }
    
    pub fn update(&mut self, delta: f32) { // delta * frame rate = 1
        self.world.update(delta);
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F1) {
            info!("Frame Time: {}, FPS: {}", delta, get_fps());
        }
    }
    
    pub fn render(&self) {
        self.world.render();
    }
    
    // pub fn quit(&mut self) {}

}