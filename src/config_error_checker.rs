use crate::config::Config;
use std::error::Error;

fn check_config(file_path: &str) -> Result<Config, Error>{
    let config: Config = serde_yaml::from_reader(std::fs::File::open("./config.yml")?)?;

    // check for required fields
    if config.cmds.is_empty() {
        eprintln!("Error: cmds are required");
    }
    if config.exec.is_empty() {
        eprintln!("Error: exec field is required");
    }

    // check for valid values
    if config.border_width.is_integer() == false{
        eprintln!("Error: border_width must be an integeir");
    }
}
