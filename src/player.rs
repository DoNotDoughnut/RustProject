use macroquad::prelude::Image;
use macroquad::prelude::KeyCode;
use macroquad::prelude::Texture2D;
use macroquad::prelude::Vec2;
use macroquad::prelude::draw_texture;
use macroquad::prelude::is_key_down;
use macroquad::prelude::load_texture_from_image;

pub struct Player {

    pos: Vec2,
    hitbox: Vec2,
    speed: f32,

    texture: Texture2D,

}

impl Player {

    pub fn update(&mut self, delta: f32) {
        if is_key_down(KeyCode::Up) {
            self.pos.y -= self.speed * delta;
            if self.pos.y < 0.0 {
                self.pos.y = 0.0;
            }
        }
        if is_key_down(KeyCode::Down) {
            self.pos.y += self.speed * delta;
            if self.pos.y + self.hitbox.y > crate::HEIGHT as f32 {
                self.pos.y = (crate::HEIGHT as f32) - self.hitbox.y;
            }
        }
        if is_key_down(KeyCode::Left) {
            self.pos.x -= self.speed * delta;
            if self.pos.x < 0.0 {
                self.pos.x = 0.0;
            }
        }
        if is_key_down(KeyCode::Right) {
            self.pos.x += self.speed * delta;
            if self.pos.x + self.hitbox.x > crate::WIDTH as _ {
                self.pos.x = (crate::WIDTH as f32) - self.hitbox.x;
            }
        }
        
    }

    pub fn render(&self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, macroquad::prelude::WHITE);
    }
    
}

impl Default for Player {
    fn default() -> Self {
        let image = Image::from_file_with_format(include_bytes!("../build/assets/player.png"), Some(image::ImageFormat::PNG)); // Loads a PNG image that is included as bytes in the executable file
        Self {
            pos: Vec2::new(10.0, 10.0),
            hitbox: Vec2::new(image.width as _, image.height as _),
            speed: 100.0,
            texture: load_texture_from_image(&image),
        }
    }
}