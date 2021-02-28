use self::player::Player;

pub mod player;

pub mod tile;
pub mod level;

pub use u16 as TileId;
pub static TILE_SIZE: u16 = 8;

use std::collections::HashMap;
use macroquad::prelude::Texture2D;
use self::level::Level;

pub struct World {

    level: Level,
    player: Player,
    textures: HashMap<TileId, Texture2D>,

    //players: HashMap<u16, PlayerData>,
    //player_texture: Texture2D

}

impl World {

    pub fn new() -> Self {
        let mut textures = HashMap::new();
        textures.insert(0, tex_bytes(include_bytes!("../../build/assets/tile0.png")).unwrap());
        textures.insert(1, tex_bytes(include_bytes!("../../build/assets/tile1.png")).unwrap());
        Self {
            level: Level::default(),
            //players: HashMap::new(),
            player: Player::default(),
            textures: textures,
        }
    }

    pub async fn load_level(&mut self) {
        // self.player.character = Some(&crate::character::NAREG);
        self.level = self::level::level_builder::LevelBuilder::debug_level();
    }

}

impl crate::Entity for World {
    fn update(&mut self, delta: f32) {
        // self.player.update(delta);


        // let mut socket = crate::net::SOCKET.lock();

        // if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::X) {
        //     socket.send(laminar::Packet::reliable_ordered( *crate::net::SERVER, b"x".to_vec(), None));
        // }

        // loop {
        //     match socket.recv() {
        //         Some(event) => match event {
        //             laminar::SocketEvent::Packet(packet) => {
        //                 let data: Result<crate::net::PlayerData, nanoserde::DeBinErr> = nanoserde::DeBin::deserialize_bin(packet.payload());
        //                 if let Ok(data) = data {
        //                     self.players.insert(packet.addr().port(), data);
        //                 }
        //             }
        //             _ => (),
        //         }
        //         None => {
        //             break;
        //         }
        //     }
        // }


    }

    fn render(&self) {
        self.level.render(&self.textures);
        // self.player.render();
        // for data in self.players.values() {
        //     crate::graphics::draw_scale(self.player.texture, data.x, data.y, crate::SCALE);
        // }
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