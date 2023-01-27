use std::fs::File;
use log::error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self};
use std::path::Path;

enum BarWidgets{
    Workspaces,
    Battery,
    Time,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    width: u16,
    height: u16,

    #[serde(default = "default_color_bg")]
    _color_bg: String,

    #[serde(default = "default_color_txt")]
    _color_txt: String,

    module_left: Vec<BarWidgets>,
    module_right: Vec<BarWidgets>,
}

impl Config {
    pub fn new(source_file: Option<&str>) -> Config {
        let home_config = &format!("{}/.config/oxide/bar_config.yml", std::env::var("HOME").unwrap());

        #[cfg(not(debug_assertions))]
        let mut paths = vec![home_config, "/etc/oxide/bar_config.yml"];

        #[cfg(debug_assertions)]
        let mut paths = vec!["./bar_config.yml", home_config, "/etc/oxide/bar_config.yml"];

        if let Some(path) = source_file {
            paths.insert(0, path);
        }

        let mut chosen_config: Option<&str> = None;
        let mut file_option: Option<File> = None;
        for path in paths.clone() {
            if Path::new(path).exists() {
                file_option = Some(File::open(path.clone()).unwrap());
                chosen_config = Some(path);
                break;
            }
        }

        match file_option {
            Some(file_option) => {
                // Reads the values from the 'bar_config' struct in bar_config.yml
                let user_config = serde_yaml::from_reader(file_option);
                match user_config {
                    Ok(config)  => return config,
                    Err(err)    => {
                        let err_msg = error!("Error in '{}': {}", chosen_config.unwrap(), err);
                        error!("ERR: {:?}", err_msg);
                    }
                }
            },
            None => {
                error!("Error: Could not find any config file. Add bar_config.yml to one of the following paths: {:?}", paths);
            }
        }
        panic!("Failed to parse config from file.");
    }
}
// Defining defualt Values
fn default_width() -> u16 {  }
fn default_height() -> u16 { 8 }
fn default_color_bg() -> String { "0x000000".to_string() } // black
fn default_color_txt() -> String { "0xFFFFFF".to_string() } // white
