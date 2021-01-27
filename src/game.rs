use macroquad::prelude::KeyCode;
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::is_key_down;
use macroquad::prelude::{Vec2, vec2};


pub struct Game {

    cube_pos: Vec2,

}

impl Game {

    pub fn new() -> Self {
        Self {
            cube_pos: vec2(50.0, 50.0),
        }
    }

    pub fn load(&mut self) {
        
    }
    
    pub fn update(&mut self, delta: f32) { // delta * frame rate = 1
        if is_key_down(KeyCode::Up) {
            self.cube_pos.y -= 100.0 * delta;
        }
        if is_key_down(KeyCode::Down) {
            self.cube_pos.y += 100.0 * delta;
        }
        if is_key_down(KeyCode::Left) {
            self.cube_pos.x -= 100.0 * delta;
        }
        if is_key_down(KeyCode::Right) {
            self.cube_pos.x += 100.0 * delta;
        }
        if macroquad::prelude::is_key_pressed(KeyCode::Key1) {
            println!("{}", delta);
        }
    }
    
    pub fn render(&self) {
        draw_rectangle(self.cube_pos.x, self.cube_pos.y, 50.0, 50.0, macroquad::prelude::RED);
    }
    
    // pub fn quit(&mut self) {}

}

