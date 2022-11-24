pub mod windowmanager;
pub mod workspace;
pub mod windowstate;

use std::error::Error;
use std::process::exit;

use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::errors::ReplyError;
use x11rb::protocol::Event;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::*;

fn handle_event<C: Connection>(
    manager: &C,
    screen_index: usize,
    event: &Event) {

    match event {
        Event::Expose(_event) => println!("-> Expose"),
        Event::UnmapNotify(_event) => println!("-> UnmapNotify"),
        Event::EnterNotify(_event) => println!("-> EnterNotify"),
        Event::ButtonPress(_event) => println!("-> ButtonPress"),
        Event::MotionNotify(_event) => println!("-> MotionNotify"),
        Event::ButtonRelease(_event) => println!("-> ButtonRelease"),
        Event::ConfigureRequest(_event) => println!("-> ConfigureRequest"),
        Event::MapRequest(_event) => println!("-> MapRequest"),
        _ => println!("-> \x1b[31mUnknown\x1b[0m"),
    };
}

fn become_wm<C: Connection>(manager: &C, screen: &Screen) -> Result<(), ReplyError> {
    let mask = ChangeWindowAttributesAux::default()
               .event_mask(
                    EventMask::SUBSTRUCTURE_REDIRECT |
                    EventMask::SUBSTRUCTURE_NOTIFY
                );

    let result = manager.change_window_attributes(
                            screen.root,
                            &mask)?
                        .check();

    if let Err(ReplyError::X11Error(ref error)) = result {
        if error.error_kind == ErrorKind::Access {
            eprintln!("Error: Access to x11 client api denied.");
            exit(1);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let (manager, screen_index) = RustConnection::connect(None)?;
    let screen = &manager.setup().roots[screen_index];

    become_wm(&manager, screen)?;

    let mut event;
    loop {
        manager.flush()?;

        event = manager.wait_for_event();
        match event {
            Ok(event) => handle_event(&manager, screen_index, &event),
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
        }
    }

    Ok(())
}
