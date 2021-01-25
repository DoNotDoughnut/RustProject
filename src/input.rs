use std::collections::HashMap;
use enum_map::{Enum, EnumMap, enum_map};
use piston_window::Button;

pub struct Input {

    pub controls: EnumMap<Control, u8>, // 0 = Key is up, 1 = Key pressed, 2 = Key held
    pub keymap: HashMap<Button, Control>, // Map keys to controls

}

impl Input {

    pub fn new() -> Self {
        use piston_window::Key;
        
        // Add key mappings to controls (with a hashmap)

        let mut keymap = HashMap::new();
        keymap.insert(Button::Keyboard(Key::X), Control::A);
        keymap.insert(Button::Keyboard(Key::Z), Control::B);
        keymap.insert(Button::Keyboard(Key::Up), Control::Up);
        keymap.insert(Button::Keyboard(Key::Down), Control::Down);
        keymap.insert(Button::Keyboard(Key::Left), Control::Left);
        keymap.insert(Button::Keyboard(Key::Right), Control::Right);

        Self {
            keymap: keymap,
            controls: enum_map! { // Set all controls to their default state (not pressed)
                Control::A => 0,
                Control::B => 0,
                Control::Up => 0,
                Control::Down => 0,
                Control::Left => 0,
                Control::Right => 0,
            },
        }
    }

    pub fn is_down(&self, control: Control) -> bool { // check if button is pressed or held
        self.controls[control] == 1 || self.controls[control] == 2
    }

}

/*
Here, #derive implements the ability to do == with other controls, 
clone/copy and avoid ownership problems like numbers, 
and make the controls able to be a map and iterated through
*/
#[derive(Debug, PartialEq, Clone, Copy, Enum, enum_iterator::IntoEnumIterator)]
pub enum Control {

    A,
    B,
    Up,
    Down,
    Left,
    Right,

}