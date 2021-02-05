use macroquad::prelude::Image;
use macroquad::prelude::Texture2D;
use macroquad::prelude::Vec2;
use macroquad::prelude::draw_texture;
use macroquad::prelude::load_texture_from_image;

use crate::input::Control;
use crate::input::is_control_down;

pub struct Player {

    pos: Vec2,
    hitbox: Vec2,
    speed: f32,

    texture: Texture2D,

}

impl crate::Entity for Player {

    fn update(&mut self, delta: f32) { // Multiplying speed by delta makes the speed constant across all framerates
        if is_control_down(Control::Up) {
            self.pos.y -= self.speed * delta;
            if self.pos.y < 0.0 {
                self.pos.y = 0.0;
            }
        }
        if is_control_down(Control::Down) {
            self.pos.y += self.speed * delta;
            if self.pos.y + self.hitbox.y > crate::HEIGHT as f32 { // Check if the hitbox of the player exceeds the bounds of the window
                self.pos.y = (crate::HEIGHT as f32) - self.hitbox.y;
            }
        }
        if is_control_down(Control::Left) {
            self.pos.x -= self.speed * delta;
            if self.pos.x < 0.0 {
                self.pos.x = 0.0;
            }
        }
        if is_control_down(Control::Right) {
            self.pos.x += self.speed * delta;
            if self.pos.x + self.hitbox.x > crate::WIDTH as _ {
                self.pos.x = (crate::WIDTH as f32) - self.hitbox.x;
            }
        }
        
    }

    fn render(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, macroquad::prelude::WHITE); // Draw the player image
    }
    
}

impl Default for Player {
    fn default() -> Self {
        let image = Image::from_file_with_format( // Load an image from bytes and specify what format it is
            include_bytes!("../../build/assets/player.png"), // Include image bytes in executable file
            Some(macroquad::prelude::ImageFormat::Png)) // Tell the image loader that the bytes are from a PNG
            .unwrap(); // Unwrap the image from the result because there should be no error (panic if there is one)
        Self {
            pos: Vec2::new(10.0, 10.0),
            hitbox: Vec2::new(image.width as _, image.height as _),
            speed: 100.0,
            texture: load_texture_from_image(&image), // Set the image to be the player image
        }
    }
}