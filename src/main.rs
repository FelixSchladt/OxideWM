pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;

use std::sync::mpsc::{channel, Sender};
use std::error::Error;
use std::thread;

use x11rb::connection::Connection;

use windowmanager::WindowManager;

#[derive(Debug)]
struct IpcEvent {
    test: String,
}


fn dbus_ipc_loop(sender: Sender<IpcEvent>) {
    loop {
        sender.send(IpcEvent { test: "test".to_string() }).unwrap();
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let (sender, receiver) = channel();

    thread::spawn(move || {
        dbus_ipc_loop(sender);
    });

    loop {
        /*
        while let Some(event) = manager.connection.borrow().poll_for_event()? {
            manager.handle_event(&event);
        }*/
        let event = manager.connection.borrow_mut().poll_for_event().unwrap();
        match event {
            Some(event) => manager.handle_event(&event),
            None => (),
        }


        let ipc_event = receiver.try_recv();
        match ipc_event {
            Ok(event) => println!("Received IPC Event: {:?}", event),
            Err(_) => (),
        }
    }

    /*
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
    }*/
}
