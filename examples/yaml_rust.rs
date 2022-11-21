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

#[derive(Debug, Serialize, Deserialize)]
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
    let f = std::fs::File::open("./examples/config.yml").expect("Could not open file.");
    let scrape_config: WmCommand = serde_yaml::from_reader(f).expect("Could not read values.");

    scrape_config.keys.push({
        WmCommand {
        keys: vec!['M', 'S', 'q'],
        command: WmCommands::Move,
        args: "M".to_string(),
        }
    });

    // println!("{:?}", scrape_config);

    // let f = std::fs::OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open("./examples/new_config.yml")
    //     .expect("Couldn't open file");
    // serde_yaml::to_writer(f, &scrape_config).unwrap();
}
