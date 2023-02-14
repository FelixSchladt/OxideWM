pub mod commands;

use commands::{IterCmd, WmCommand, WmCommandArgument};
use log::{error, info};
use oxide_common::ipc::commands::WmCommands;
use oxide_common::ipc::state::ConfigDto;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::path::Path;
use std::process::Command;

use crate::workspace::workspace_layout::WorkspaceLayout;

const DEFAULT_BORDER_WIDTH: u32 = 3;

const DEFAULT_BORDER_COLOR: &str = "0xFFFFFF"; // white

const DEFAULT_BORDER_FOCUS_COLOR: &str = "0x000000"; // black

const DEFAULT_GAP: u32 = 10;

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

    #[serde(default = "default_gap")]
    pub gap: u32,

    #[serde(default = "default_default_layout")]
    pub default_layout: WorkspaceLayout,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            cmds: default_cmds(),
            iter_cmds: default_icmds(),
            exec: default_exec(),
            exec_always: default_exec_always(),
            border_width: default_border_width(),
            border_color: default_border_color(),
            border_focus_color: default_border_focus_color(),
            gap: default_gap(),
            default_layout: default_default_layout(),
        }
    }
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
        Command::new("notify-send")
            .args([
                "--urgency=critical",
                "'Failed to load config, using defaults!'",
            ])
            .output()
            .ok();
        Config::default()
    }

    pub fn to_dto(&self) -> ConfigDto {
        let cmds = self.cmds.iter().map(|cmd| cmd.to_dto()).collect();
        ConfigDto {
            cmds: cmds,
            exec: self.exec.clone(),
            exec_always: self.exec_always.clone(),
            border_width: self.border_width.clone(),
            border_color: self.border_color.clone(),
            border_focus_color: self.border_focus_color.clone(),
            gap: self.gap.clone(),
        }
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
    vec![WmCommand {
        keys: vec!["A".to_string(), "t".to_string()],
        commands: vec![WmCommandArgument {
            command: WmCommands::Exec,
            args: Some("kitty".to_string()),
        }],
    }]
}
fn default_icmds() -> Vec<IterCmd> {
    vec![]
}
fn default_exec() -> Vec<String> {
    Vec::<String>::new()
}
fn default_exec_always() -> Vec<String> {
    Vec::<String>::new()
}
fn default_border_width() -> u32 {
    DEFAULT_BORDER_WIDTH
}
fn default_border_color() -> String {
    DEFAULT_BORDER_COLOR.to_string()
}
fn default_border_focus_color() -> String {
    DEFAULT_BORDER_FOCUS_COLOR.to_string()
}
fn default_gap() -> u32 {
    DEFAULT_GAP
}

fn default_default_layout() -> WorkspaceLayout {
    WorkspaceLayout::VerticalStriped
}
