use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub is_alpha: bool,
}

impl Color {
    fn new(hex: String) -> Color {
        let red = u8::from_str_radix(&hex[1..3], 16).unwrap() as f64 / 255.0;
        let green = u8::from_str_radix(&hex[3..5], 16).unwrap() as f64 / 255.0;
        let blue = u8::from_str_radix(&hex[5..7], 16).unwrap() as f64 / 255.0;
        if hex.len() == 9 {
            let alpha = u8::from_str_radix(&hex[7..9], 16).unwrap() as f64 / 255.0 as f64;
            Color {
                red,
                green,
                blue,
                alpha,
                is_alpha: true,
            }
        } else {
            Color {
                red,
                green,
                blue,
                alpha: 1.0,
                is_alpha: false,
            }
        }
    }

    pub fn rgb(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }

    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Color::new(s))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_width")]
    pub width: u16,
    #[serde(default = "default_height")]
    pub height: u16,

    #[serde(default = "default_color_bg", deserialize_with = "deserialize_color")]
    pub color_bg: Color,

    #[serde(
        default = "default_color_txt_inactive",
        deserialize_with = "deserialize_color"
    )]
    pub color_txt_inactive: Color,

    #[serde(default = "default_color_txt", deserialize_with = "deserialize_color")]
    pub color_txt: Color,
}

impl Config {
    pub fn new(width: u16) -> Config {
        let home_config = &format!(
            "{}/.config/oxide/bar_config.yml",
            std::env::var("HOME").unwrap()
        );

        #[cfg(not(debug_assertions))]
        let paths: Vec<&str> = vec![home_config, "/etc/oxide/bar_config.yml"];

        #[cfg(debug_assertions)]
        let paths: Vec<&str> = vec!["./bar_config.yml", home_config, "/etc/oxide/bar_config.yml"];

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
                let user_config: Result<Config, serde_yaml::Error> =
                    serde_yaml::from_reader(file_option);
                match user_config {
                    Ok(config) => {
                        let mut config: _ = config;
                        config.width = width;
                        config.height = 30; //This is hardcoded for now
                        println!("Using config file: {:?}", config);
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
fn default_color_bg() -> Color {
    Color::new("#000009".to_string())
} // black

fn default_color_txt_inactive() -> Color {
    Color::new("#606060".to_string())
} // white

fn default_color_txt() -> Color {
    Color::new("#FFFFFF".to_string())
} // white
fn default_width() -> u16 {
    0
}
fn default_height() -> u16 {
    0
}
