use std::fs::File;
use log::error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self};
use std::path::Path;

use crate::eventhandler::commands::WmCommands;

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where D: Deserializer<'de>,
{
    let args = Option::<String>::deserialize(deserializer)?;
    let args = args.unwrap_or("".to_string());
    if args.is_empty() || args == "None" {
        Ok(None)
    } else {
        Ok(Some(args))
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WmCommand {
    pub keys: Vec<String>,
    pub command: WmCommands,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub args: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_cmds")]
    pub cmds: Vec<WmCommand>,

    #[serde(default = "default_exec")]
    pub exec: Vec<String>,

    #[serde(default = "default_exec_always")]
    pub exec_always: Vec<String>,

    #[serde(default = "default_border_width")]
    pub border_width: u32,

    #[serde(default = "default_border_color")]
    pub border_color: u32,

    #[serde(default = "default_border_focus_color")]
    pub border_focus_color: u32,

    #[serde(default = "default_gap")]
    pub gap: u32,
}


impl Config {
    pub fn new(source_file: Option<&str>) -> Config {
        let home_config = &format!("{}/.config/oxide/config.yml", std::env::var("HOME").unwrap());

        #[cfg(not(debug_assertions))]
        let mut paths = vec![home_config, "/etc/oxide/config.yml"];

        #[cfg(debug_assertions)]
        let mut paths = vec!["./config.yml", home_config, "/etc/oxide/config.yml"];

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
                // Reads the values from the 'config' struct in config.yml
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
                error!("Error: Could not find any config file. Add config.yml to one of the following paths: {:?}", paths);
            }
        }
        panic!("Failed to parse config from file.");
    }
}

// Defining default values
fn default_cmds() -> Vec<WmCommand> {
    vec![WmCommand{
        keys: vec!["A".to_string(), "t".to_string()],
        command: WmCommands::Exec,
        args: Some("kitty".to_string())
    }]
}

fn default_exec() -> Vec<String> {
    Vec::<String>::new()
}

fn default_exec_always() -> Vec<String> {
    Vec::<String>::new()
}

fn default_border_width() -> u32 { 3 }
fn default_border_color() -> u32 { 0xFFFFFF } // white
fn default_border_focus_color() -> u32 { 0x000000 } // black
fn default_gap() -> u32 { 3 }
