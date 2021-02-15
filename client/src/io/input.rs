use serde::{Serialize, Deserialize};

pub mod keyboard;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Control { // List of controls for game

    A,
    Up,
    Down,
    Left,
    Right,

}

pub fn is_control_pressed(control: Control) -> bool {
    if keyboard::is_key_pressed(&control) {
        return true;
    } else {
        return false; // Return false if no keys for control or any keys mapped to the control were pressed
    }
    
}

pub fn is_control_down(control: Control) -> bool { // Look to function above for details on how this function works
    if keyboard::is_key_down(&control) {
        return true;
    } else {
        return false;
    }
}