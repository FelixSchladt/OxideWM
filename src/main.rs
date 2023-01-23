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
pub mod atom;
pub mod constants;
pub mod common;
pub mod logging;

#[cfg(test)]
pub mod test;

use std::sync::{Arc, Mutex};

use std::sync::mpsc::channel;
use std::thread;
use std::{cell::RefCell, rc::Rc};

use config::Config;
use serde_json::Result;
use log::{error, trace};

use crate::{
    logging::init_logger,
    windowmanager::WindowManager,
    eventhandler::EventHandler,
    keybindings::KeyBindings,
    eventhandler::events::IpcEvent,
    ipc::zbus_serve,
};

fn main() -> Result<()> {
    #[cfg(test)]
    test::run_and_exit();

    init_logger();

    let mut config = Rc::new(RefCell::new(Config::new(None)));
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
            eventhandler.handle_event(&event);
        } else {
            if let Some(error) = result.err(){
                error!("Error retreiving Event from Window manager {:?}", error);
            }
        }

        if let Ok(event) = wm_receiver.try_recv() {
            if event.status {
                let wm_state = eventhandler.window_manager.get_state();
                let j = serde_json::to_string(&wm_state)?;
                trace!("IPC status request");
                wm_sender.send(j).unwrap();
            } else {
                eventhandler.handle_ipc_event(event);
            }
        }

        if eventhandler.window_manager.restart {
            config = Rc::new(RefCell::new(Config::new(None)));
            keybindings = KeyBindings::new(&config.borrow());

            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(&keybindings, config.clone());
        }
    }
}
