pub mod windowmanager;
pub mod workspace;
pub mod windowstate;

use windowmanager::WindowManager;

use std::error::Error;
use std::process::exit;

use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::errors::ReplyError;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::*;

fn handle_event(
    manager: &WindowManager,
    event: &Event) {

    match event {
        Event::Expose(_event) => println!("Expose"),
        Event::UnmapNotify(_event) => println!("UnmapNotify"),
        Event::EnterNotify(_event) => println!("EnterNotify"),
        Event::ButtonPress(_event) => println!("ButtonPress"),
        Event::MotionNotify(_event) => println!("MotionNotify"),
        Event::ButtonRelease(_event) => println!("ButtonRelease"),
        Event::ConfigureRequest(_event) => println!("ConfigureRequest"),
        Event::MapRequest(_event) => println!("MapRequest"),
        _ => println!("-> \x1b[33mUnknown Event\x1b[0m"),
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let manager = WindowManager::new();

    let mut event;
    loop {
        manager.connection.flush()?;

        event = manager.connection.wait_for_event();
        match event {
            Ok(event) => handle_event(&manager, &event),
            Err(error) => {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m {}", error);
                break;
            }
        }
    }

    Ok(())
}
