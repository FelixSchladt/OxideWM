pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
pub mod ipc;

use std::sync::mpsc::channel;

use std::thread;

use serde_json::Result;

use x11rb::connection::Connection;

use std::sync::{Arc, Mutex};

use crate::windowmanager::{WindowManager, IpcEvent};
use ipc::zbus_serve;

fn main() -> Result<()> {
    let mut manager = WindowManager::new();

    let (ipc_sender, wm_receiver) = channel::<IpcEvent>();
    let (wm_sender, ipc_receiver) = channel::<String>(); 

    let ipc_sender_mutex = Arc::new(Mutex::new(ipc_sender));
    let ipc_receiver_mutex = Arc::new(Mutex::new(ipc_receiver));

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve(ipc_sender_mutex, ipc_receiver_mutex)).unwrap();
    });

    loop {
        let event = manager.connection.borrow_mut().poll_for_event().unwrap();
        match event {
            Some(event) => manager.handle_event(&event),
            None => (),
        }
        let ipc_event = wm_receiver.try_recv();
        match ipc_event {
            Ok(event) => {
                if event.status {
                    let j = serde_json::to_string(&manager)?;
                    println!("IPC status request");
                    wm_sender.send(j).unwrap();
                 } else {
                    manager.handle_ipc_event(event);
                }
            },
            Err(_) => (),
        }
    }
}
