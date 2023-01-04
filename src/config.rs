use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WmCommands {
    Move, //args: left, up, right, down
    Focus,
    Resize,
    Quit, // Quit the window manager
    Kill, // Kill the focused window
    Restart, // Restart the window manager
    MoveToWorkspace,
    GoToWorkspace,
    MoveToWorkspaceAndFollow,
    Exec,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WmCommand {
    pub keys: Vec<String>,
    pub command: WmCommands,
    pub args: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub border_color: String,
    
    #[serde(default = "default_border_focus_color")]
    pub border_focus_color: String,

    #[serde(default = "default_gap")]
    pub gap: u8,
}

impl Config {
    pub fn new() -> Config {
        //simulate_config()
        // Opens the config.yaml file.
        let f = File::open("./config.yml").expect("Could not open file.");
        // Reads the Values from the 'config' struct in config.yml 
        let user_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        println!("{:?}", user_config);
        user_config
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
fn default_border_color() -> String { "white".to_string() }
fn default_border_focus_color() -> String { "black".to_string() }
fn default_gap() -> u8 { 3 }


