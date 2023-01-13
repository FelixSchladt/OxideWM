#![deny(clippy::pedantic)]

pub mod eventhandler;
pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
pub mod ipc;

use std::sync::{Arc, Mutex};

use std::sync::mpsc::channel;
use std::thread;
use std::{cell::RefCell, rc::Rc};

use config::Config;
use log::error;
use serde_json::Result;

use crate::{
    windowmanager::{WindowManager, IpcEvent},
    eventhandler::EventHandler,
    keybindings::KeyBindings,
    ipc::zbus_serve,
};


fn main() -> Result<()> {
    let mut config = Rc::new(RefCell::new(Config::new()));
    let mut keybindings = KeyBindings::new(&config.borrow());

    let mut manager = WindowManager::new(&keybindings, config.clone());
    let mut eventhandler = EventHandler::new(&mut manager, &keybindings);

    let (ipc_sender, wm_receiver) = channel::<IpcEvent>();
    let (wm_sender, ipc_receiver) = channel::<String>();


    let ipc_sender_mutex = Arc::new(Mutex::new(ipc_sender));
    let ipc_receiver_mutex = Arc::new(Mutex::new(ipc_receiver));

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve(ipc_sender_mutex, ipc_receiver_mutex)).unwrap();
    });

    loop {
        let result = eventhandler.window_manager.poll_for_event();
        if let Ok(Some(event)) = result {
            eventhandler.handle_event(&event)
        } else {
            error!("Error retreiving Event from Window manager {}", result.err().unwrap());
        }

        if let Ok(event) = wm_receiver.try_recv() {
            if event.status {
                let wm_state = eventhandler.window_manager.get_state();
                let j = serde_json::to_string(&wm_state)?;
                println!("IPC status request");
                wm_sender.send(j).unwrap();
            } else {
                eventhandler.handle_ipc_event(event);
            }
        }

        if eventhandler.window_manager.restart {
            config = Rc::new(RefCell::new(Config::new()));
            keybindings = KeyBindings::new(&config.borrow());
    
            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(&keybindings, config.clone());
        }
    }
}
