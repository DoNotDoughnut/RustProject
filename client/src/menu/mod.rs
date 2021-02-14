use crate::gui::button::Button;
use crate::input::Control;
use crate::input::is_control_pressed;

pub struct MainMenu {

    selected_button: u8,
    play_button: Button,
    option_button: Button,
    quit_button: Button,

}

impl MainMenu {

    pub fn new() -> Self {
        Self {
            selected_button: 0,
            play_button: Button::new(
                40.0,
                40.0,
                200.0,
                100.0,
                macroquad::prelude::BEIGE,
                Some("Play"),
            ),
            option_button: Button::new(
                40.0,
                160.0,
                200.0,
                100.0,
                macroquad::prelude::BLUE,
                Some("Options"),
            ),
            quit_button: Button::new(
                40.0, 
                280.0,
                200.0,
                100.0,
                macroquad::prelude::RED,
                Some("Quit")
            ),
        }
    }

    pub fn update(&mut self, _delta: f32) {
        if macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) { // Test for left click
            let pos = macroquad::prelude::mouse_position(); // Get mouse position
            // pos.0 /= crate::SCALE as f32; // Scale the mouse position
            // pos.1 /= crate::SCALE as f32;
            if self.play_button.x <= pos.0 && pos.0 < self.play_button.x + self.play_button.width { // If the x coordinate of the mouse position is in the range of the buttons
                if self.play_button.y <= pos.1 && pos.1 < self.play_button.y + self.play_button.height { // Test if the mouse y position is over the first button
                    self.selected_button = 0;
                    self.click();
                } else if self.option_button.y <= pos.1 && pos.1 < self.option_button.y + self.option_button.height {
                    self.selected_button = 1;
                    self.click();
                } else if self.quit_button.y <= pos.1 && pos.1 < self.quit_button.y + self.quit_button.height {
                    self.selected_button = 2;
                    self.click();
                }
            }
        }
        if is_control_pressed(Control::Up) {
            if self.selected_button > 0 {
                self.selected_button -= 1;
            }
        }
        if is_control_pressed(Control::Down) {
            if self.selected_button < 2 {
                self.selected_button += 1;
            }
        }
        if is_control_pressed(Control::A) {
            self.click();
        }
    }

    pub fn render(&self) { // Render the buttons
        self.play_button.render(self.selected_button == 0);
        self.option_button.render(self.selected_button == 1);
        self.quit_button.render(self.selected_button == 2);
    }

    fn click(&mut self) { // Button click actions
        match Buttons::from(self.selected_button) {
            Buttons::Play => {
                crate::game::change_game_state(crate::game::GameState::World);
            }
            Buttons::Options => {
                macroquad::prelude::info!("Not implemented yet!");
            },
            Buttons::Quit => {
                crate::quit(); // Quit the game
            }
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Buttons {
    
    Play,
    Options,
    Quit,

}

impl From<u8> for Buttons {
    fn from(id: u8) -> Self {
        match id {
            1 => Buttons::Options,
            2 => Buttons::Quit,
            _ => Buttons::Play,
        }
    }
}