use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use super::PersistentData;
use super::input::Control;

static FILE: &str = "config.json";

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

#[async_trait::async_trait(?Send)]
impl PersistentData for Configuration {
    
    async fn load() -> Self {
        match macroquad::prelude::load_file("config.json").await {
            Ok(bytes) => match std::str::from_utf8(&bytes) {
                Ok(content) => match serde_json::from_str(content) {
                    Ok(config) => config,
                    Err(err) => {
                        warn!("Could not read configuration JSON with error {}", err);
                        Self::saved_default().await
                    }
                }
                Err(err) => {
                    warn!("Could not parse configuration JSON to string with error {}", err);
                    Self::saved_default().await
                }
            }
            Err(err) => {
                warn!("Could not find configuration JSON file with error {}", err);
                Self::saved_default().await
            }
        }
    }

    async fn save(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(data) => match File::create(FILE) {
                Ok(mut file) => match file.write_all(data.as_bytes()) {
                    Ok(()) => info!("Successfully wrote configuration file!"),
                    Err(err) => warn!("Could not write to configuration file with error {}", err),
                }
                Err(err) => warn!("Could not create configuration file at {} with error {}", FILE, err),
            }
            Err(err) => warn!("Could not serialize configuration with error {}", err),
        }
    }

}

impl Configuration {

    async fn saved_default() -> Self {
        let default = Self::default();
        default.save().await;
        return default;
    }

    pub fn on_load(&self) {
        *crate::io::input::keyboard::KEYBOARD_CONTROLS.write() = self.controls.clone();
    }

}