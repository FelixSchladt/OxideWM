use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub enum BarWidgets {
    Workspaces,
    Battery,
    Time,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub width: u16,
    pub height: u16,

    #[serde(default = "default_color_bg")]
    pub color_bg: String,

    #[serde(default = "default_color_txt")]
    pub color_txt: String,

    pub module_left: Vec<BarWidgets>,
    pub module_right: Vec<BarWidgets>,
}

impl Config {
    pub fn new(width: u16) -> Config {
        let home_config = &format!(
            "{}/.config/oxide/bar_config.yml",
            std::env::var("HOME").unwrap()
        );

        #[cfg(not(debug_assertions))]
        let paths = vec![home_config, "/etc/oxide/bar_config.yml"];

        #[cfg(debug_assertions)]
        let paths = vec!["./bar_config.yml", home_config, "/etc/oxide/bar_config.yml"];

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
                let user_config: Result<Config, serde_yaml::Error> = serde_yaml::from_reader(file_option);
                match user_config {
                    Ok(config) => {
                        let mut config: _ = config;
                        config.width = width;
                        config.height = 30; //This is hardcoded for now
                        return config;
                    }
                    Err(err) => {
                        eprintln!("Error in '{}': {}", chosen_config.unwrap(), err);
                    }
                }
            }
            None => {
                eprintln!("Error: Could not find any config file. Add bar_config.yml to one of the following paths: {:?}", paths);
            }
        }
        panic!("Failed to parse config from file.");
    }
}
// Defining defualt Values
fn default_color_bg() -> String {
    "0x000000".to_string()
} // black
fn default_color_txt() -> String {
    "0xFFFFFF".to_string()
} // white
