use ahash::AHashSet;
use parking_lot::RwLock;

use ahash::AHashMap;
use macroquad::prelude::KeyCode;

lazy_static::lazy_static! {
    static ref KEY_CONTROLS: RwLock<AHashMap<Control, AHashSet<KeyCode>>> = RwLock::new(AHashMap::new());
}

pub(crate) fn set_controls() {
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
pub enum Control {

    Up,
    Down,
    Left,
    Right,

}