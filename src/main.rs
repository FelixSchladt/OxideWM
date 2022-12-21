pub mod windowmanager;
pub mod workspace;
pub mod windowstate;

use windowmanager::WindowManager;
use std::error::Error;
use x11rb::connection::Connection;
use x11rb::protocol::Event;



fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let mut event;
    println!("WindorManager: {:?}", manager.screeninfo);

    loop {
        manager.connection.flush()?;

        event = manager.connection.wait_for_event();
        match event {
            Ok(event) =>  {println!("event: {:?}", event); manager.handle_event(&event);},
            Err(error) => {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m {}", error);
                break;
            }
        }
    }

    Ok(())
}
