use crate::world::TileId;

#[derive(Debug, Clone, Copy)]
pub struct Tile {

    pub tile_id: TileId,
    pub solid: bool,

}