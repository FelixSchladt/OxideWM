use std::collections::HashMap;
use std::process::{Command, Stdio};

use crate::config::{Config, WmCommands};

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
#[derive(Clone)]
pub struct KeyCode {
    pub mask: u16,
    pub code: u8,
}


#[derive(Debug)]
#[derive(Clone)]
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

//TODO Maybe move this to a separate file
pub fn placeholder(args: Option<String>) -> (Option<String>, Option<String>) {
    match args {
        Some(args) => println!("Placeholder function called with args: {}", args),
        None => println!("Placeholder function called without args"),
    }
    //TODO decide if tuple return is necessary 
    //Why did I implement it this way??? Well IDK
    return (None, None);
}

//TODO Maybe move this to a separate file
pub fn exec_user_command(args: Option<String>) -> (Option<String>, Option<String>) {
    match args {
        Some(args) => {
            let mut args = args.split_whitespace();
            let command = args.next().unwrap();
            let args = args.collect::<Vec<&str>>().join(" ");
            if args.is_empty() {
                Command::new(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            } else {
                Command::new(command)
                    .arg(args)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            }.unwrap();
            return (None, None);
        },
        None => panic!("User command called without args"),
    }
}

#[derive(Debug)]
pub struct KeyBindings {
    pub events_map: HashMap<u8, Vec<KeyEvent>>,
    pub events_vec: Vec<KeyEvent>,
}

impl KeyBindings {
    pub fn new(config: &Config) -> KeyBindings {
        let mut keybindings = KeyBindings {
            events_map: HashMap::new(),
            events_vec: Vec::new(),
        };
        
        let keymap = keycodes_map();

        //add wm commands
        for cmd in &config.cmds {
            let keycode = convert_to_keycode(&mut cmd.keys.clone(), &keymap);
            let kevent = KeyEvent {
                keycode: keycode.clone(),
                args: cmd.args.clone(),
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
            };
            keybindings.events_vec.push(kevent.clone());
            keybindings.events_map
                .entry(keycode.code)
                .or_insert(Vec::new())
                .push(kevent);
        }
        
        //add user commands
        for ucmd in &config.user_cmds {
            let keycode = convert_to_keycode(&mut ucmd.keys.clone(), &keymap);
            let kevent = KeyEvent {
                keycode: keycode.clone(),
                args: Some(ucmd.args.clone()),
                event: exec_user_command,
            };
            keybindings.events_vec.push(kevent.clone());
            keybindings.events_map
                .entry(keycode.code)
                .or_insert(Vec::new())
                .push(kevent);
        }

        keybindings
    }
}
