use serde::Deserialize;
use std::env;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Core {
    pub note_dir: String,
    pub open_cmd: String,
    pub always_open: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub core: Core,
}

pub fn load_config() -> Config {
    let home_path = match env::var("HOME") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not get home path: {}", e);
            exit(1);
        }
    };
    let filename = home_path + "/.config/nojo/nojo.toml";

    let contents = match fs::read_to_string(filename.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read config file: {}", e);
            exit(1);
        }
    };

    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from '{}", filename);
            exit(1)
        }
    };
    data
}
