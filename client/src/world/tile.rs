use super::TileId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {

    pub tile_id: TileId,
    pub solid: bool,

}