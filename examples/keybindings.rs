use std::collections::HashMap;
use std::process::Command;

#[derive(Debug)]
enum WmCommands {
    Move,
    Resize,
    Quit,
    Restart,
    MoveToWorkspace,
    GoToWorkspace,
    MoveToWorkspaceAndFollow,
}

#[derive(Debug)]
struct WmCommand {
    keys: Vec<char>,
    command: WmCommands,
    args: String,
}

#[derive(Debug)]
struct UserCommand {
    keys: Vec<char>,
    command: String,
}

#[derive(Debug)]
struct Config {
    cmds: Vec<WmCommand>,
    /*
    user_cmds: Vec<UserCommand>,
    exec: Vec<String>,
    exec_always: Vec<String>,
    border_with: u8,
    border_color: String,
    border_focus_color: String,
    titlebar: bool,
    gap: u8,
    */
}


fn simulate_config() -> Config {
    let mut config = Config {
        cmds: Vec::new(),
    };
    config.cmds.push(WmCommand {
        keys: vec!['S', 'q'],
        command: WmCommands::Quit,
        args: String::new(),
    });
    config.cmds.push(WmCommand {
        keys: vec!['C', 'r'],
        command: WmCommands::Restart,
        args: String::new(),
    });
    config.cmds.push(WmCommand {
        keys: vec!['A', 'm'],
        command: WmCommands::Move,
        args: String::new(),
    });
    config.cmds.push(WmCommand {
        keys: vec!['M', 'r'],
        command: WmCommands::Resize,
        args: String::new(),
    });

    return config;
}

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
    mask: u16,
    code: u8,
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

fn main() {
    let keycodes = keycodes_map();
    let config = simulate_config();

    for cmd in config.cmds {
        let mut keys = cmd.keys.clone();
        let parse = convert_to_keycode(&mut keys, &keycodes);
        println!("Parse: {:?}\n", parse);
    }
}
