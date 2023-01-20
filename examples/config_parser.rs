use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
enum WmCommands {
    Move,
    Resize,
    Quit,
    Restart,
    MoveToWorkspace,
    GoToWorkspace,
    MoveToWorkspaceAndFollow,
}

#[derive(Debug, Serialize, Deserialize,)]
struct WmCommand {
    keys: Vec<char>,
    command: WmCommands,
    args: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserCommand {
    keys: Vec<char>,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {

    #[serde(default = "default_modifier")]
    modifier: char,

    #[serde(default = "default_cmds")]
    cmds: Vec<WmCommand>,

    #[serde(default = "default_user_cmds")]
    user_cmds: Vec<UserCommand>,

    #[serde(default = "default_exec")]
    exec: Vec<String>,

    #[serde(default = "default_exec_always")]
    exec_always: Vec<String>,

    #[serde(default = "default_border_with")]
    border_with: u8,
    
    #[serde(default = "default_border_color")]
    border_color: String,
    
    #[serde(default = "default_border_focus_color")]
    border_focus_color: String,
    
    #[serde(default = "default_titlebar")]
    titlebar: bool,
    
    #[serde(default = "default_gap")]
    gap: u8,
}

// Defining default values
fn default_modifier() -> char {'M'}
fn default_cmds() -> Vec<WmCommand> {
    vec![WmCommand{
        keys: vec!['H', 'A'], 
        command: WmCommands::Move, 
        args: "1".to_string()
    }]
}
fn default_user_cmds() -> Vec<UserCommand> {
    vec![UserCommand {
        keys: vec!['A', 'B'], 
        command: "S".to_string()
    }]
}

fn default_exec() -> Vec<String> {
    vec!["L".to_string(), "O".to_string(), "L".to_string()]
}

fn default_exec_always() -> Vec<String> {
    vec!["H".to_string(), "I".to_string()]
}

fn default_border_with() -> u8 { 3 }
fn default_border_color() -> String { "white".to_string() }
fn default_border_focus_color() -> String { "black".to_string() }
fn default_titlebar() -> bool { false }
fn default_gap() -> u8 { 3 }

fn main() {
    
    // Opens the config.yaml file.
    let f = File::open("./examples/config.yml").expect("Could not open file.");
    // Reads the Values from the 'config' struct in config.yml 
    let user_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", user_config);

}
