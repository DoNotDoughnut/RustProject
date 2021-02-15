use std::collections::{HashMap, HashSet};

use macroquad::prelude::KeyCode;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};

use super::input::Control;

#[derive(Deserialize, Serialize)]
pub struct Configuration {

    pub controls: HashMap<Control, HashSet<KeyCode>>,

}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            controls: super::input::keyboard::default(),
        }
    }
}

impl Configuration {
    
    pub async fn load_or_default() -> Configuration {
        match macroquad::prelude::load_file("config.json").await {
            Ok(bytes) => {
                match std::str::from_utf8(&bytes) {
                    Ok(content) => {
                        let result: Result<Configuration, serde_json::Error> = serde_json::from_str(content);
                        match result {
                            Ok(config) => config,
                            Err(err) => {
                                warn!("Could not read configuration JSON with error {}", err);
                                Configuration::default()
                            }
                        }
                    }
                    Err(err) => {
                        warn!("Could not parse configuration JSON to string with error {}", err);
                        Configuration::default()
                    }
                }
            }
            Err(err) => {
                warn!("Could not find configuration JSON file with error {}", err);
                Configuration::default()
            }
        }
    }

    pub fn on_load(&self) {
        *crate::io::input::keyboard::KEYBOARD_CONTROLS.write() = self.controls.clone();
    }

}