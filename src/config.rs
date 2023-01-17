use std::fs::File;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self};
use std::process;
use std::path::Path;

use crate::eventhandler::commands::WmCommands;

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
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
    pub border_width: u8,
    
    #[serde(default = "default_border_color")]
    pub border_color: i32,
    
    #[serde(default = "default_border_focus_color")]
    pub border_focus_color: i32,

    #[serde(default = "default_gap")]
    pub gap: u8,
}


impl Config {
    pub fn new() -> Config {
        let mut file_option: Option<File> = None;
        let mut paths = vec![ "~/.config/oxidewm/config.yml", "/etc/oxidewm/config.yml"];
        #[cfg(not(release))]
        paths.insert(0, "./config.yml");
        let mut chosen_config: Option<&str> = None;
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
                        let err_msg = format!("Error in '{}': {}", chosen_config.unwrap(), err);
                        //TODO: Write this error to a log file
                        println!("ERR: {:?}", err_msg);
                    }
                }
            },
            None => {
                eprintln!("Error: Could not find any config file. Add config.yml to one of the following paths: {:?}", paths);
            }
        }
        process::exit(-1);
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
    vec!["L".to_string(), "O".to_string(), "L".to_string()]
}

fn default_exec_always() -> Vec<String> {
    vec!["H".to_string(), "I".to_string()]
}

fn default_border_width() -> u8 { 3 }
fn default_border_color() -> i32 { 0xFFFFFF } // white
fn default_border_focus_color() -> i32 { 0x000000 } // black
fn default_gap() -> u8 { 3 }