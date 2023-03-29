// Get standard config from env
use std::env;

pub struct Config {

}

impl Config {
    pub fn get(item : &str) -> String {
        // Attempt to get a config item from env
        match std::env::var(item) {
            Ok(e) => e,
            Err(_) => Config::get_default_config(item),
        }
    }

    pub fn get_default_config(item : &str) -> String {
        match item {
            GRAFANA_URL => "http://localhost:3000/".to_owned(),
        }
    }
}