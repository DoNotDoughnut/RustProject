use super::TileId;
use nanoserde::{SerJson, DeJson};

#[derive(Debug, Clone, Copy, SerJson, DeJson)]
pub struct Tile {

    pub tile_id: TileId,
    pub solid: bool,

}