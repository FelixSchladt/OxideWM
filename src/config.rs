
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

/*
#[derive(Debug)]
pub enum ModifierKey {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

impl From<ModifierKey> for u16 {
    fn from(key: ModifierKey) -> u16 {
        (match key {
            ModifierKey::Shift  =>  1,
            ModifierKey::Ctrl   =>  4,
            ModifierKey::Alt    =>  8,
            ModifierKey::Meta   =>  64,
        }) as u16
    }
}

impl TryFrom<char> for ModifierKey {
    type Error = &'static str;
    fn try_from(key: char) -> Result<Self, Self::Error> {
        match key {
            'C'     => Ok(ModifierKey::Ctrl),
            'A'     => Ok(ModifierKey::Alt),
            'S'     => Ok(ModifierKey::Shift),
            'M'     => Ok(ModifierKey::Meta),
            _       => Err("Invalid modifier key"),
        }
    }
}

#[derive(Debug)]
pub struct KeyCode {
    pub mask: u16,
    pub code: u8,
}


#[derive(Debug)]
pub struct KeyEvent {
    pub keycode: KeyCode,
    pub args: Option<String>,
    pub event: fn(Option<String>)->(Option<String>, Option<String>),
}

fn keycodes_map() -> HashMap<String, u8> {
    let output = Command::new("xmodmap")
        .arg("-pke")
        .output()
        .expect("xmodmap failed tor run")
        .stdout;
    let m = String::from_utf8(output).unwrap();
    let mut keycodes_map: HashMap<String, u8> = HashMap::new();
    for line in m.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() > 3 {
                keycodes_map.insert(words[3].to_string(), words[1].parse().unwrap());
            }
        }
    return keycodes_map; 
}

fn keyname_to_keycode(keyname: &str, keymap: &HashMap<String, u8>) -> u8 {
    return *keymap.get(keyname).unwrap_or_else(|| panic!("Key {} has no corresponding keysym", keyname));
}

//TODO ERROR handling
fn convert_to_keycode(keys: &mut Vec<char>, keymap: &HashMap<String, u8>) -> KeyCode {
    let mut mask: u16 = 0;
    let keyname = keys.pop().unwrap();
    let code = keyname_to_keycode(&keyname.to_string(), keymap);

    //Accepts multiple modifiers but only one key
    for modifier in keys {
        mask = mask | u16::from(ModifierKey::try_from(*modifier).unwrap());
    }

    return KeyCode {
        mask: mask, //bitmask of the modifiers
        code: code, //keycode
    };
}

//Example function for even calling
fn placeholder(args: Option<String>) -> (Option<String>, Option<String>) {
    match args {
        Some(args) => println!("Placeholder function called with args: {}", args),
        None => println!("Placeholder function called without args"),
    }
    return (None, None);
}


pub fn get_keyevents_vec() -> Vec<KeyEvent> {
    let mut keyevents: Vec<KeyEvent> = Vec::new();
    let keymap = keycodes_map();
    let config = simulate_config();
    for cmd in config.cmds {
        let keycode = convert_to_keycode(&mut cmd.keys.clone(), &keymap);
        keyevents.push(KeyEvent {
            keycode: keycode,
            args: cmd.args,
            event: match cmd.command {
                //TODO: Replace placeholder with actual functions
                WmCommands::Quit => placeholder,
                WmCommands::Restart => placeholder,
                WmCommands::Move => placeholder,
                WmCommands::Resize => placeholder,
                WmCommands::MoveToWorkspace => placeholder,
                WmCommands::GoToWorkspace => placeholder,
                WmCommands::MoveToWorkspaceAndFollow => placeholder,
            },
        });
    }
    return keyevents;
}

//TODO: maybe return a struct with hashmap and vec to prevent calling the xmodmap command twice...
pub fn get_keyevents() -> HashMap<u8, Vec<KeyEvent>> {
    let mut keyevents: HashMap<u8, Vec<KeyEvent>> = HashMap::new();
    for keyevent in get_keyevents_vec() {
        let keyevents_vec = keyevents.entry(keyevent.keycode.code).or_insert(Vec::new());
        keyevents_vec.push(keyevent);
    }
    return keyevents;
}
*/
