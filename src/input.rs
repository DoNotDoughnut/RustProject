use std::collections::HashMap;
use enum_map::{Enum, EnumMap, enum_map};
use piston_window::Button;

pub struct Input {

    pub controls: EnumMap<Control, ControlState>,
    pub buttonmap: HashMap<Button, Control>, // Map keys to controls

}

impl Input {

    pub fn new() -> Self {
        use piston_window::Key;
        
        // Add button mappings to controls (with a hashmap)

        let mut buttonmap = HashMap::new();

        // Keyboard buttons

        buttonmap.insert(Button::Keyboard(Key::X), Control::A);
        buttonmap.insert(Button::Keyboard(Key::Z), Control::B);
        buttonmap.insert(Button::Keyboard(Key::Up), Control::Up);
        buttonmap.insert(Button::Keyboard(Key::Down), Control::Down);
        buttonmap.insert(Button::Keyboard(Key::Left), Control::Left);
        buttonmap.insert(Button::Keyboard(Key::Right), Control::Right);

        use ControlState::Up;

        Self {
            buttonmap: buttonmap,
            controls: enum_map! { // Set all controls to their default state (not pressed)
                Control::A => Up,
                Control::B => Up,
                Control::Up => Up,
                Control::Down => Up,
                Control::Left => Up,
                Control::Right => Up,
            },
        }
    }

    pub fn is_down(&self, control: Control) -> bool { // check if button is pressed or held
        self.controls[control] == ControlState::Pressed || self.controls[control] == ControlState::Held
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ControlState {

    Up,
    Pressed,
    Held,

}