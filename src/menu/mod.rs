use crate::game::GAME_STATE;
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
                20.0,
                20.0,
                100.0,
                50.0,
                macroquad::prelude::BEIGE,
                Some("Play"),
            ),
            option_button: Button::new(
                20.0,
                80.0,
                100.0,
                50.0,
                macroquad::prelude::BLUE,
                Some("Options"),
            ),
            quit_button: Button::new(
                20.0, 
                140.0,
                100.0,
                50.0,
                macroquad::prelude::RED,
                Some("Quit")
            )
        }
    }

    pub fn update(&mut self, _delta: f32) {
        if macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
            let mut pos = macroquad::prelude::mouse_position();
            pos.0 /= crate::SCALE as f32;
            pos.1 /= crate::SCALE as f32;
            if 20.0 <= pos.0 && pos.0 < 120.0 {
                if 20.0 <= pos.1 && pos.1 < 70.0 {
                    self.selected_button = 0;
                    self.click();
                } else if 80.0 <= pos.1 && pos.1 < 130.0 {
                    self.selected_button = 1;
                    self.click();
                } else if 140.0 <= pos.1 && pos.1 < 190.0 {
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

    pub fn render(&self) {
        self.play_button.render(self.selected_button == 0);
        self.option_button.render(self.selected_button == 1);
        self.quit_button.render(self.selected_button == 2);
    }

    fn click(&mut self) {
        match Buttons::from(self.selected_button) {
            Buttons::Play => {
                unsafe {
                    GAME_STATE = crate::game::GameState::World;
                }
            }
            Buttons::Options => {
                macroquad::prelude::info!("Not implemented yet!");
            },
            Buttons::Quit => {
                crate::quit();
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