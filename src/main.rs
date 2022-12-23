pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;

use windowmanager::WindowManager;
use std::error::Error;
use x11rb::connection::Connection;


fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let mut event;

    loop {
        event = manager.connection.borrow_mut().wait_for_event();
        match event {
            Ok(event) =>  {println!("event: {:?}", event); manager.handle_event(&event);},
            Err(error) => {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m {}", error);
                break;
            }
        }

        manager.connection.borrow_mut().flush()?;
    }

    Ok(())
}
