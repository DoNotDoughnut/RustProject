use std::collections::{HashMap, HashSet};
use macroquad::prelude::KeyCode;

pub fn is_control_pressed(control: Control) -> bool {
    if crate::DESKTOP { // If device is a computer
        if let Some(keys) = KEY_CONTROLS.get(&control) { // Get the keys for the control pressed
            for key in keys { // Iterate through the keys
                if macroquad::prelude::is_key_pressed(*key) { // Check if the key was pressed
                    return true; // Return true if the key was pressed
                }
            }
        }
    }
    return false; // Return false if not a desktop, no keys for control or any keys mapped to the control were pressed
}

pub fn is_control_down(control: Control) -> bool {
    if crate::DESKTOP {
        if let Some(keys) = KEY_CONTROLS.get(&control) {
            for key in keys {
                if macroquad::prelude::is_key_down(*key) {
                    return true;
                }
            }
        }
    }
    return false;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Control { // List of controls for game

    A,
    Up,
    Down,
    Left,
    Right,

}

lazy_static::lazy_static! {
    // AHashMaps are faster hashmaps
    // RwLock allows the HashMap to be written to and read from multiple threads (but we dont use more than 1 thread, its just a safety guarantee)
    // Note: DashMap crate could be better for this
    static ref KEY_CONTROLS: HashMap<Control, HashSet<KeyCode>> = { // create a global hashmap for the controls
        let mut controls = HashMap::new();

         // Set the controls for the game

        controls.insert(Control::A, set_of(&[KeyCode::X]));
        controls.insert(Control::Up, set_of(&[KeyCode::Up, KeyCode::W]));
        controls.insert(Control::Down, set_of(&[KeyCode::Down, KeyCode::S]));
        controls.insert(Control::Left, set_of(&[KeyCode::Left, KeyCode::A]));
        controls.insert(Control::Right, set_of(&[KeyCode::Right, KeyCode::D]));

        return controls; // Return the controls (the brackets make the variable like a function I think)
    };    
}

fn set_of(codes: &[KeyCode]) -> HashSet<KeyCode> { // Helper method, creates a set of keys from an array
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}