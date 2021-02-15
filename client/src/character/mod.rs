use macroquad::prelude::Image;
use macroquad::prelude::Texture2D;
use macroquad::prelude::Vec2;

lazy_static::lazy_static! {
    pub static ref NAREG: Character = Character::default();
}

pub struct Character {

    pub hitbox: Vec2,
    pub speed: f32,

    pub texture: Texture2D,

}

impl Default for Character {
    fn default() -> Self {
        let image = Image::from_file_with_format( // Load an image from bytes and specify what format it is
            include_bytes!("../../build/assets/player.png"), // Include image bytes in executable file
            Some(macroquad::prelude::ImageFormat::Png)) // Tell the image loader that the bytes are from a PNG
            .unwrap(); // Unwrap the image from the result because there should be no error (panic if there is one)
        Self {
            hitbox: Vec2::new(image.width as _, image.height as _),
            speed: 100.0,
            texture: macroquad::prelude::load_texture_from_image(&image), // Set the image to be the player image
        }
    }
}