use std::collections::HashMap;
use macroquad::prelude::Texture2D;
use u16 as MapSize;
use super::TILE_SIZE;
use super::TileId;
use super::level_builder::LevelBuilder;
use super::tile::Tile;

type TexMap = HashMap<TileId, Texture2D>;

#[derive(Default)]
pub struct Level {

    pub width: MapSize,
    pub height: MapSize,
    pub tiles: Vec<Tile>,

}

impl Level { // Window size = 40 tiles x 22.5 tiles

    pub fn debug_level() -> Self {
        let mut builder = LevelBuilder::new(40, 22);

        builder.add_row(Tile {
            tile_id: 0,
            solid: false,
        });

        builder.add_rows(Tile {
            tile_id: 1,
            solid: true,
        }, 20);

        builder.add_row(Tile {
            tile_id: 0,
            solid: false,
        });

        builder.build()
    }

    // pub fn new(path: String) -> Self {
    //     Self {
    //         ..Default::default()
    //     }
    // }

    pub fn render(&self, textures: &TexMap) {
        for y in 0..self.height {
            //let row = y * self.width;
            for x in 0..self.width {
                let params = self.texture_from(textures, x, y);
                // macroquad::texture::draw_texture_ex(params.0, params.1 * 2.0, params.2 * 2.0 + 4.0, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
                //     dest_size: Some(macroquad::prelude::vec2(16.0, 16.0)),
                //     ..Default::default()
                // });
                crate::graphics::draw(params.0, params.1, params.2 + 2.0);
            }
        }
    }

    fn texture_from(&self, textures: &TexMap, x: u16, y: u16) -> (Texture2D, f32, f32) {
        (
            *textures.get(&self.tile_at(x, y).tile_id).unwrap(),
            (x * TILE_SIZE) as f32,
            (y * TILE_SIZE) as f32
        )        
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Tile {
        self.tiles[(x + y * self.width) as usize]
    }

}