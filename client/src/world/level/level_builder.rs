use super::Level;
use crate::world::tile::Tile;

pub struct LevelBuilder {
    level: Level,
}

impl LevelBuilder {

    pub fn debug_level() -> Level {
        let mut builder = LevelBuilder::new(40, 22);

        builder.add_row(Tile {
            tile_id: 0,
            solid: true,
        });

        builder.add_rows(Tile {
            tile_id: 1,
            solid: false,
        }, 20);

        builder.add_row(Tile {
            tile_id: 0,
            solid: true,
        });

        builder.add_tile(Tile {
            tile_id: 0,
            solid: true,
        }, 10, 10);

        builder.build()
    }

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            level: Level {
                width: width,
                height: height,
                tiles: Vec::with_capacity((width*height) as usize),
            }
        }
    }

    pub fn add_tile(&mut self, tile: Tile, x: usize, y: usize) {
        self.level.tiles[x + y * self.level.width as usize] = tile;
    }

    pub fn add_rows(&mut self, tile: Tile, rows: usize) {
        for _ in 0..rows {
            self.add_row(tile);
        }
    }

    pub fn add_row(&mut self, tile: Tile) {
        self.level.tiles.append(&mut vec![tile; self.level.width as usize]);
    }

    pub fn build(mut self) -> Level {
        let size = (self.level.width*self.level.height) as usize;
        while self.level.tiles.len() - 1 > size {
            self.level.tiles.pop();
        }
        self.level
    }

}