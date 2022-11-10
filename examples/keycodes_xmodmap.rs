use std::process::Command;
use std::collections::HashMap;

fn main() {
    let output = Command::new("xmodmap")
        .arg("-pke")
        .output()
        .expect("xmodmap failed tor run")
        .stdout;
    let m = String::from_utf8(output).unwrap();
    let mut keycodes_map: HashMap<u16, String> = HashMap::new();
    for line in m.lines() {
            //println!("{}",element);
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() > 3 {
                //println!("Code: {}, Name: {}", words[1], words[3]);
                keycodes_map.insert(words[1].parse().unwrap(), words[3].to_string());
            }
        }
    println!("{:?}", keycodes_map);
}

