//use x11rb::connection::Connection;
use std::error::Error;
use log::info;

fn main() -> Result<(), Box<dyn Error>>{
    let (connection, screen_index) = x11rb::connect(None)?;
    println!("Connection successful (Screen: {})", screen_index);

    Ok(())
}
