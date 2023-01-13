/* use crate::config::Config;
use std::error::Error; */

//  Nothing is currently working
/* fn check_config(file_path: &str) -> Result<Config, Box<dyn Error>>{
    let config: Config = serde_yaml::from_reader(std::fs::File::open("./config.yml")?)?;

    // check for required fields
    let variable = config;
    for variable in config.into_iter() {

         if variable.is_empty() {
            eprintln!("Error: {} is a required field!", variable)
        } 
    }
        if config.cmds.is_empty() {
            eprintln!("Error: cmds are required");
        }

        if config.exec.is_empty() {
            eprintln!("Error: exec field is required");
        }

        // check for valid values
        /* if config.border_width as u8 == false{
            eprintln!("The order witdh must be an integer");
        } */

    Ok(config)
    } */
