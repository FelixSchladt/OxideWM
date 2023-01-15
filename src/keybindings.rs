use std::collections::HashMap;
use std::process::Command;

use log::debug;
use x11rb::protocol::xproto::{KeyPressEvent, ModMask};

use crate::{
    config::Config,
    eventhandler::commands::WmCommands,
};

#[derive(Debug)]
pub enum ModifierKey {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

impl From<ModifierKey> for u16 {
    fn from(key: ModifierKey) -> u16 {
        match key {
            ModifierKey::Shift  =>  1  as u16,
            ModifierKey::Ctrl   =>  4  as u16,
            ModifierKey::Alt    =>  8  as u16,
            ModifierKey::Meta   =>  64 as u16,
        }
    }
}

impl TryFrom<String> for ModifierKey {
    type Error = &'static str;
    fn try_from(key: String) -> Result<Self, Self::Error> {
        match key.as_str() {
            "C"     => Ok(ModifierKey::Ctrl),
            "A"     => Ok(ModifierKey::Alt),
            "S"     => Ok(ModifierKey::Shift),
            "M"     => Ok(ModifierKey::Meta),
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
    pub event: WmCommands,
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
fn convert_to_keycode(keys: &mut Vec<String>, keymap: &HashMap<String, u8>) -> KeyCode {
    let mut mask: u16 = 0;
    let keyname = keys.pop().unwrap(); //Only one not modifier key is accepted
    let code = keyname_to_keycode(&keyname.to_string(), keymap);

    //Accepts multiple modifiers but only one key
    for modifier in keys {
        mask = mask | u16::from(ModifierKey::try_from(modifier.clone()).unwrap());
    }

    return KeyCode {
        mask: mask, //bitmask of the modifiers
        code: code, //keycode
    };
}

#[derive(Debug)]
pub struct KeyBindings {
    pub events_map: HashMap<u8, Vec<KeyEvent>>,
    pub events_vec: Vec<KeyEvent>,
}

impl KeyBindings {
    #[must_use]
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
                event: cmd.command.clone(),
            };
            keybindings.events_vec.push(kevent.clone());
            keybindings.events_map
                .entry(keycode.code)
                .or_default()
                .push(kevent);
        }
        keybindings
    }

    #[must_use]
    pub fn retreive_cmd(&self, event: &KeyPressEvent) -> Option<KeyEvent>{
        let keys = if let Some(k) = self.events_map.get(&event.detail) { k } else {
             debug!("Found no matching key for event {}", event.detail);
             return None;
         };

        //NOTE: IF you get the error above, this is probably cause by an inconsistency
        // in the Connection. Most likely you did something with the connection that
        // left it in a weird state. This **must not be** directly connected to this
        // function. Maybe a flush helps but check if there is something else wrong
        // with your changes. I experienced this a couple of times and it always was
        // quite strange and hard to find. Ask for help if you can't find the problem.

        for key in keys.clone() {
            let state = u16::from(event.state);
            if state == key.keycode.mask
            || state == key.keycode.mask | u16::from(ModMask::M2) {
                debug!("Key: {:?}", key);
                return Some(key);
            }
        }
        return None;
    }
}
