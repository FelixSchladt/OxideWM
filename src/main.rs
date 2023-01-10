pub mod windowmanager;
pub mod eventhandler;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::error::Error;
use std::{thread};

use config::Config;
use log::error;
use x11rb::{
    connection::Connection,
    protocol::xproto::{
        ModMask,
        ConnectionExt,
        GrabMode,
    },
};

use crate::{
    windowmanager::WindowManager,
    eventhandler::EventHandler,
    keybindings::KeyBindings
};

#[derive(Debug)]
struct IpcEvent {
    test: String,
}


fn dbus_ipc_loop(sender: Sender<IpcEvent>) {
    loop {
        //sender.send(IpcEvent { test: "test".to_string() }).unwrap();
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}


fn grab_keys(windowmanager: &mut WindowManager,keybindings: &KeyBindings) -> Result<(), Box<dyn Error>> {
    //TODO check if the the screen iterations should be merged
    for screen in windowmanager.connection.borrow().setup().roots.iter() {
        for modifier in [0, u16::from(ModMask::M2)] {
            for keyevent in keybindings.events_vec.iter() {
                windowmanager.connection.borrow().grab_key(
                    false,
                    screen.root,
                    (keyevent.keycode.mask | modifier).into(),
                    keyevent.keycode.code,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                )?;
            }
        }
    }
    windowmanager.connection.borrow().flush().expect("failed to flush rust connection");
Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let config = Rc::new(RefCell::new(Config::new()));
    let keybindings = KeyBindings::new(&config.borrow());
    
    let mut manager = WindowManager::new();
    let mut eventhandler = EventHandler::new(&mut manager, &keybindings);
    
    grab_keys(eventhandler.window_manager,&keybindings).expect("failed to grab keys");


    let (sender, receiver) = channel();

    thread::spawn(move || {
        dbus_ipc_loop(sender);
    });

    loop {
        let result = eventhandler.window_manager.poll_for_event();
        if(!result.is_err()){
            let event = result.unwrap();
            match event {
                Some(event) => eventhandler.handle_event(&event),
                None => (),
            }
        }else {
            error!("Error retreiving Event from Window manager {}", result.err().unwrap());
        }

        let ipc_event = receiver.try_recv();
        match ipc_event {
            Ok(event) => println!("Received IPC Event: {:?}", event),
            Err(_) => (),
        }
    }
}
