use self::player::Player;
use self::level::Level;

mod level;
mod tile;
mod player;

pub mod level_builder;

use std::collections::HashMap;
use macroquad::prelude::Texture2D;
pub use u16 as TileId;
pub static TILE_SIZE: u16 = 8;

pub struct World {

    level: Level,
    player: Player,
    textures: HashMap<TileId, Texture2D>,

}

impl World {

    pub fn new() -> Self {
        let mut textures = HashMap::new();
        textures.insert(0, tex_bytes(include_bytes!("../../build/assets/tile0.png")).unwrap());
        textures.insert(1, tex_bytes(include_bytes!("../../build/assets/tile1.png")).unwrap());
        Self {
            level: Level::default(),
            player: Player::default(),
            textures: textures,
        }
    }

    pub async fn load_level(&mut self) {
        self.level = Level::debug_level();
    }

}

impl crate::Entity for World {
    fn update(&mut self, delta: f32) {
        self.player.update(delta);
    }

    fn render(&self) {
        self.level.render(&self.textures);
        self.player.render();
    }
}

fn tex_bytes(bytes: &[u8]) -> Option<Texture2D> {
    match macroquad::prelude::Image::from_file_with_format(bytes, Some(macroquad::prelude::ImageFormat::Png)) {
        Ok(ref image) => {
            let tex = macroquad::prelude::load_texture_from_image(image);
            macroquad::prelude::set_texture_filter(tex, macroquad::prelude::FilterMode::Nearest);
            return Some(tex);
        },
        Err(err) => {
            macroquad::prelude::warn!("Could not get texture from bytes with error {}", err);
            return None;
        }
    }
}