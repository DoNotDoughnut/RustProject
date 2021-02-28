// #[cfg(not(target_arch = "wasm32"))]
// pub mod nakama;

// #[cfg(target_arch = "wasm32")]
// #[rename("nakama")]
mod wasm_nakama;
pub use wasm_nakama::*;

pub mod syncronizer;
pub mod global_events;
pub mod remote_player;

// #[derive(DeBin, SerBin)]
// pub struct PlayerData {

//     pub x: f32,
//     pub y: f32,

// }

pub struct MatchData {
    pub data: Vec<u8>,
    pub opcode: i32,
    pub user_id: String,
}

#[allow(dead_code)]
pub enum Event {
    Join(String),
    Leave(String),
}
