use log::{error, info};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self};
use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::eventhandler::commands::WmCommands;

const DEFAULT_CMDS: Vec<WmCommand> = vec![WmCommand {
    keys: vec!["A".to_string(), "t".to_string()],
    commands: vec![WmCommandArgument {
        command: WmCommands::Exec,
        args: Some("kitty".to_string()),
    }],
}];

const DEFAULT_ICDMS: Vec<IterCmd> = vec![];

const DEFAULT_EXEC: Vec<String> = Vec::<String>::new();

const DEFAULT_EXEC_ALWAYS: Vec<String> = Vec::<String>::new();

const DEFAULT_BORDER_WIDTH: u32 = 3;

const DEFAULT_BORDER_COLOR: String = "0xFFFFFF".to_string(); // white

const DEFAULT_BORDER_FOCUS_COLOR: String = "0x000000".to_string(); // black

const DEFAULT_GAP: u32 = 10;

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

fn deserialize_u32_gap<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let args = u32::deserialize(deserializer);
    println!("Args {:?}", args);
    match args {
        Ok(value) => Ok(value),
        Err(error) => {
            //println!("Error: {:?}", error);
            /*error!(
                "Wrong Datatype for {} field in line {}. Entered {} but {} is expected.",
                error.path, error.line, error.invalid_type, error.expected
            );*/
            error!("Wrong datatype: {}", error.to_string());
            //let msg = error.message.unwrap();
            //println!("Error: {} at ", msg);
            return Ok(8);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WmCommandArgument {
    pub command: WmCommands,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub args: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WmCommand {
    pub keys: Vec<String>,
    pub commands: Vec<WmCommandArgument>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IterCmd {
    pub iter: Vec<String>,
    pub command: WmCommand,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_cmds")]
    pub cmds: Vec<WmCommand>,

    #[serde(default = "default_icmds")]
    pub iter_cmds: Vec<IterCmd>,

    #[serde(default = "default_exec")]
    pub exec: Vec<String>,

    #[serde(default = "default_exec_always")]
    pub exec_always: Vec<String>,

    #[serde(default = "default_border_width")]
    pub border_width: u32,

    #[serde(default = "default_border_color")]
    pub border_color: String,

    #[serde(default = "default_border_focus_color")]
    pub border_focus_color: String,

    #[serde(default = "default_gap", deserialize_with = "deserialize_u32_gap")]
    pub gap: u32,
}

impl Config {
    pub fn new(source_file: Option<&str>) -> Config {
        let home_config = &format!(
            "{}/.config/oxide/config.yml",
            std::env::var("HOME").unwrap()
        );

        #[cfg(not(debug_assertions))]
        let mut paths: Vec<&str> = vec![home_config, "/etc/oxide/config.yml"];

        #[cfg(debug_assertions)]
        let mut paths: Vec<&str> = vec!["./config.yml", home_config, "/etc/oxide/config.yml"];

        if let Some(path) = source_file {
            paths.insert(0, path);
        }

        let mut chosen_config: Option<&str> = None;
        for path in paths.clone() {
            if Path::new(path).exists() {
                chosen_config = Some(path);
                break;
            }
        }

        match chosen_config {
            Some(config_path) => {
                info!("using config {config_path}");

                // Reads the values from the 'config' struct in config.yml
                let config_file = File::open(config_path).unwrap();
                let user_config: Result<Config, serde_yaml::Error> =
                    serde_yaml::from_reader(config_file);

                match user_config {
                    Ok(mut config) => {
                        config.parse_iter_cmds();
                        return config;
                    }
                    Err(err) => {
                        let err_msg = error!("Error in '{}': {}", config_path, err);
                        error!("ERR: {:?}", err_msg);
                    }
                }
            }
            None => {
                error!("Error: Could not find any config file. Add config.yml to one of the following paths: {:?}", paths);
            }
        }
        panic!("Failed to parse config from file.");
    }

    fn parse_iter_cmds(&mut self) {
        for icmd in &self.iter_cmds {
            for i in &icmd.iter {
                let mut cmd = icmd.command.clone();
                for key in cmd.keys.iter_mut() {
                    *key = key.replace("$VAR", i);
                }
                for command in cmd.commands.iter_mut() {
                    if let Some(args) = &mut command.args {
                        *args = args.replace("$VAR", i);
                    }
                }
                self.cmds.push(cmd);
            }
        }
    }
}

// Defining default values
fn default_cmds() -> Vec<WmCommand> {
    DEFAULT_CMDS
}
fn default_icmds() -> Vec<IterCmd> {
    DEFAULT_ICDMS
}
fn default_exec() -> Vec<String> {
    DEFAULT_EXEC
}
fn default_exec_always() -> Vec<String> {
    DEFAULT_EXEC_ALWAYS
}
fn default_border_width() -> u32 {
    DEFAULT_BORDER_WIDTH
}
fn default_border_color() -> String {
    DEFAULT_BORDER_COLOR
}
fn default_border_focus_color() -> String {
    DEFAULT_BORDER_FOCUS_COLOR
}
fn default_gap() -> u32 {
    DEFAULT_GAP
}
