use macroquad::prelude::warn;
use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct Configuration {

}

impl Default for Configuration {
    fn default() -> Self {
        Self {}
    }
}

impl Configuration {
    
    pub async fn load_or_default() -> Configuration {
        match macroquad::prelude::load_file("config.json").await {
            Ok(bytes) => {
                match std::str::from_utf8(&bytes) {
                    Ok(content) => {
                        let result: Result<Configuration, nanoserde::DeJsonErr> = nanoserde::DeJson::deserialize_json(content);
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

}