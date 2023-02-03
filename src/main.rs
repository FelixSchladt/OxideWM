#![deny(clippy::pedantic)]

pub mod atom;
pub mod auxiliary;
pub mod common;
pub mod config;
pub mod constants;
pub mod eventhandler;
pub mod ipc;
pub mod keybindings;
pub mod logging;
pub mod screeninfo;
pub mod setup;
pub mod windowmanager;
pub mod windowstate;
pub mod workspace;

#[cfg(test)]
#[path = "../test/mod.rs"]
mod test;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use config::Config;
use log::info;
use serde_json::Result;
use std::{cell::RefCell, rc::Rc};
use x11rb::rust_connection::RustConnection;

use crate::{
    eventhandler::events::EventType, eventhandler::EventHandler, ipc::zbus_serve,
    keybindings::KeyBindings, windowmanager::WindowManager,
};

fn get_status_channel() -> (Arc<Mutex<Sender<String>>>, Arc<Mutex<Receiver<String>>>) {
    let (status_sender, status_receiver) = channel::<String>();
    let status_sender_mutex = Arc::new(Mutex::new(status_sender));
    let status_receiver_mutex = Arc::new(Mutex::new(status_receiver));
    (status_sender_mutex, status_receiver_mutex)
}

fn get_event_channel() -> (
    Arc<Mutex<Sender<EventType>>>,
    Arc<Mutex<Receiver<EventType>>>,
) {
    let (event_sender, event_receiver) = channel::<EventType>();
    let event_sender_mutex = Arc::new(Mutex::new(event_sender));
    let event_receiver_mutex = Arc::new(Mutex::new(event_receiver));
    (event_sender_mutex, event_receiver_mutex)
}

fn start_zbus_thread(
    event_sender_mutex: Arc<Mutex<Sender<EventType>>>,
    status_receiver_mutex: Arc<Mutex<Receiver<String>>>,
) {
    info!("starting zbus serve");
    thread::spawn(move || {
        // as seperate thread to speed up boot time
        async_std::task::block_on(zbus_serve(event_sender_mutex, status_receiver_mutex)).unwrap();
    });
}

fn start_x_event_thread(
    connection: Arc<RustConnection>,
    event_sender_mutex: Arc<Mutex<Sender<EventType>>>,
) {
    info!("starting x event proxy");
    thread::spawn(move || {
        WindowManager::run_event_proxy(connection, event_sender_mutex);
    });
}

fn main() -> Result<()> {
    logging::init_logger();

    let mut config = Rc::new(RefCell::new(Config::new(None)));
    let mut keybindings = KeyBindings::new(&config.borrow());
    let connection = setup::connection::get_connection(&keybindings.clone());

    let mut manager = WindowManager::new(connection.clone(), config.clone());
    let binding = keybindings.clone();
    let mut eventhandler = EventHandler::new(&mut manager, &binding);

    let (event_sender_mutex, event_receiver_mutex) = get_event_channel();
    let (status_sender_mutex, status_receiver_mutex) = get_status_channel();

    start_zbus_thread(event_sender_mutex.clone(), status_receiver_mutex.clone());
    start_x_event_thread(connection.clone(), event_sender_mutex.clone());

    loop {
        info!("starting event loop");
        eventhandler.run_event_loop(event_receiver_mutex.clone(), status_sender_mutex.clone());

        if eventhandler.window_manager.restart {
            setup::connection::ungrab_keys(connection.clone(), &keybindings).unwrap();

            config = Rc::new(RefCell::new(Config::new(None)));
            keybindings = KeyBindings::new(&config.borrow());
            setup::connection::grab_keys(connection.clone(), &keybindings.clone()).unwrap();

            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(config.clone());
        } else {
            break;
        }
    }
    Ok(())
}
