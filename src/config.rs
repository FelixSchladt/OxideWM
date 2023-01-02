
#[derive(Debug)]
#[derive(Clone)]
pub enum WmCommands {
    Move,
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
    pub keys: Vec<char>,
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
        keys: vec!['A', 'S', 'e'],
        command: WmCommands::Quit,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!['A', 'S', 'r'],
        command: WmCommands::Restart,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!['A', 'S', 'q'],
        command: WmCommands::Kill,
        args: None,
    });
    config.cmds.push(WmCommand {
        keys: vec!['A', 't'],
        command: WmCommands::Exec,
        args: Some("kitty".to_string()),
    });

    return config;
}
