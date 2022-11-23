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
    cmds: Vec<WmCommand>,
    user_cmds: Vec<UserCommand>,
    exec: Vec<String>,
    exec_always: Vec<String>,
    border_with: u8,
    border_color: String,
    border_focus_color: String,
    titlebar: bool,
    gap: u8,
}

fn main() {
    
    // Setting default values
    let default_config = Config {
        cmds: vec![WmCommand{
            keys: vec!['H', 'A'], 
            command: WmCommands::Move, 
            args: "1".to_string()
        }],

        user_cmds: vec![UserCommand {
            keys: vec!['A', 'B'],
            command: "S".to_string()
        }],
        
        exec : vec!["H".to_string(), "I".to_string()],
        exec_always : vec!["L".to_string(), "O".to_string(), "L".to_string()],
        border_with : 5,
        border_color : "black".to_string(),
        border_focus_color : "white".to_string(),
        titlebar : false,
        gap : 9,
    };

    println!("{:?}", default_config);

    let f = std::fs::File::open("./examples/config.yml").expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", scrape_config);

}
