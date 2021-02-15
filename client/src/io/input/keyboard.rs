use std::collections::HashMap;
use std::collections::HashSet;
use parking_lot::RwLock;

use macroquad::prelude::KeyCode;

use super::Control;

pub fn is_key_pressed(control: &Control) -> bool {
    if let Some(keys) = KEYBOARD_CONTROLS.read().get(control) { // Get the keys for the control pressed
        for key in keys { // Iterate through the keys
            if macroquad::prelude::is_key_pressed(*key) { // Check if the key was pressed
                return true; // Return true if the key was pressed
            }
        }
    }
    return false; // Return false if not a desktop, no keys for control or any keys mapped to the control were pressed
}

pub fn is_key_down(control: &Control) -> bool { // Look to function above for details on how this function works
    if let Some(keys) = KEYBOARD_CONTROLS.read().get(control) {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    return false;
}

lazy_static::lazy_static! {
    pub static ref KEYBOARD_CONTROLS: RwLock<HashMap<Control, HashSet<KeyCode>>> = RwLock::new(default()); // create a global hashmap for the controls 
}

pub fn default() -> HashMap<Control, HashSet<KeyCode>> {
    let mut controls = HashMap::new();

        // Set the controls for the game

    controls.insert(Control::A, keyset(&[KeyCode::X]));
    controls.insert(Control::Up, keyset(&[KeyCode::Up, KeyCode::W]));
    controls.insert(Control::Down, keyset(&[KeyCode::Down, KeyCode::S]));
    controls.insert(Control::Left, keyset(&[KeyCode::Left, KeyCode::A]));
    controls.insert(Control::Right, keyset(&[KeyCode::Right, KeyCode::D]));

    return controls; // Return the controls (the brackets make the variable like a function I think)
}

fn keyset(codes: &[KeyCode]) -> HashSet<KeyCode> { // Helper method, creates a set of keys from an array
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}