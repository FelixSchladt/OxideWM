
#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Clone)]
#[derive(Debug)]
pub struct WmCommand {
    pub keys: Vec<String>,
    pub command: WmCommands,
    pub args: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub cmds: Vec<WmCommand>,
    /*
    exec: Vec<String>,
    exec_always: Vec<String>,
    border_with: u8,
    border_color: String,
    border_focus_color: String,
    titlebar: bool,
    gap: u8,
    */
}

impl Config {
    pub fn new() -> Config {
        simulate_config()
    }
}


fn simulate_config() -> Config {
    let mut config = Config {
        cmds: Vec::new(),
    };
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "S".to_string(), "e".to_string()],
        command: WmCommands::Quit,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "S".to_string(), "r".to_string()],
        command: WmCommands::Restart,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "S".to_string(), "q".to_string()],
        command: WmCommands::Kill,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "t".to_string()],
        command: WmCommands::Exec,
        args: Some("kitty".to_string()),
    });
    //right
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "l".to_string()],
        command: WmCommands::Focus,
        args: Some("right".to_string()),
    });
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "S".to_string(), "l".to_string()],
        command: WmCommands::Move,
        args: Some("right".to_string()),
    });
    //left
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "S".to_string(), "h".to_string()],
        command: WmCommands::Move,
        args: Some("left".to_string()),
    });
    config.cmds.push(WmCommand {
        keys: vec!["A".to_string(), "h".to_string()],
        command: WmCommands::Focus,
        args: Some("left".to_string()),
    });
    //TODO: up and down are not implemented currently 
    // blocked by issue https://github.com/DHBW-FN/OxideWM/issues/25

    return config;
}
