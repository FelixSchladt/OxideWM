
#[derive(Debug)]
#[derive(Clone)]
pub enum WmCommands {
    Move,
    Resize,
    Quit,
    Restart,
    MoveToWorkspace,
    GoToWorkspace,
    MoveToWorkspaceAndFollow,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct WmCommand {
    pub keys: Vec<char>,
    pub command: WmCommands,
    pub args: Option<String>,
}

#[derive(Debug)]
pub struct UserCommand {
    pub keys: Vec<char>,
    //arg fiels is the command executed by the shell
    //TODO: Maybe rename this field to something more descriptive
    pub args: String,
}

#[derive(Debug)]
pub struct Config {
    pub cmds: Vec<WmCommand>,
    pub user_cmds: Vec<UserCommand>,
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
        user_cmds: Vec::new(),
    };
    config.cmds.push(WmCommand {
        keys: vec!['A', 'r'],
        command: WmCommands::Quit,
        args: Some("1".to_string()),
    });
    config.cmds.push(WmCommand {
        keys: vec!['C', 'r'],
        command: WmCommands::Restart,
        args: Some("2".to_string()),
    });
    config.cmds.push(WmCommand {
        keys: vec!['S', 'A', 'r'],
        command: WmCommands::Move,
        args: Some("3".to_string()),
    });
    config.cmds.push(WmCommand {
        keys: vec!['M', 'r'],
        command: WmCommands::Resize,
        args: Some("4".to_string()),
    });
    config.user_cmds.push(UserCommand {
        keys: vec!['A', 't'],
        args: "kitty".to_string(),
    });

    return config;
}
