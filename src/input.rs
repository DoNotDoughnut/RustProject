use ahash::AHashSet;
use parking_lot::RwLock;

use ahash::AHashMap;
use macroquad::prelude::KeyCode;

lazy_static::lazy_static! {
    // AHashMaps are faster hashmaps
    // RwLock allows the HashMap to be written to and read from multiple threads (but we dont use more than 1 thread, its just a safety guarantee)
    // Note: DashMap crate is better for this
    static ref KEY_CONTROLS: RwLock<AHashMap<Control, AHashSet<KeyCode>>> = RwLock::new(AHashMap::new()); // create a global hashmap for the controls
}

pub(crate) fn set_controls() { // Map keys to controls
    let mut controls = KEY_CONTROLS.write();
    controls.insert(Control::Up, set_of(&[KeyCode::Up, KeyCode::W]));
    controls.insert(Control::Down, set_of(&[KeyCode::Down, KeyCode::S]));
    controls.insert(Control::Left, set_of(&[KeyCode::Left, KeyCode::A]));
    controls.insert(Control::Right, set_of(&[KeyCode::Right, KeyCode::D]));
}

fn set_of(codes: &[KeyCode]) -> AHashSet<KeyCode> {
    let mut set = AHashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}

pub fn is_control_down(control: Control) -> bool {
    if let Some(keys) = KEY_CONTROLS.read().get(&control) {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    return false;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Control { // List of controls for game

    Up,
    Down,
    Left,
    Right,

}