use super::level::Level;
use super::tile::Tile;

pub struct LevelBuilder {
    level: Level,
}

impl LevelBuilder {

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            level: Level {
                width: width,
                height: height,
                tiles: Vec::with_capacity((width*height) as usize),
            }
        }
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