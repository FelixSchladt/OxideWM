use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Config {}

fn main() {
    // Opens the config.yaml file.
    let f = File::open("./examples/config.yml").expect("Could not open file.");
    // Reads the Values from the 'config' struct in config.yml
    let user_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", user_config);
}
