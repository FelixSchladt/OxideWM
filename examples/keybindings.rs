use std::collections::HashMap;
use std::process::Command;

/*
pub enum ModifierKey {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

pub struct KeyCode {
    /// The held modifier mask
    pub mask: KeyCodeMask,
    /// The key code that was held
    pub code: KeyCodeValue,
}
*/

//This does not seem to work in the Xephyr environment
fn keycodes_map() -> HashMap<String, u16> {
    let output = Command::new("xmodmap")
        .arg("-pke")
        .output()
        .expect("xmodmap failed tor run")
        .stdout;
    let m = String::from_utf8(output).unwrap();
    let mut keycodes_map: HashMap<String, u16> = HashMap::new();
    for line in m.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() > 3 {
//                println!("Code: {}, Name: {}", words[1], words[3]);
                keycodes_map.insert(words[3].to_string(), words[1].parse().unwrap());
            }
        }
    //println!("{:?}", keycodes_map);
    //println!("Keycodes map created");
    return keycodes_map; 
}

fn keyname_to_keycode(keyname: &str, keymap: &HashMap<String, u16>) -> u16 {
    return *keymap.get(keyname).unwrap_or_else(|| panic!("Key {} has no corresponding keysym", keyname));
}

fn main() {
    let keycodes = keycodes_map();
    println!("Keyname to keycode: {}", keyname_to_keycode("a", &keycodes));
}
