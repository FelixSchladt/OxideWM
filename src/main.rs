pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
pub mod ipc;

use std::sync::mpsc::sync_channel;

use std::error::Error;
use std::thread;

use x11rb::connection::Connection;

use crate::windowmanager::{WindowManager, IpcEvent};
use ipc::zbus_serve;

fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let size: usize = 1000;
    let (ipc_sender, wm_receiver) = sync_channel::<IpcEvent>(size);
    //let (wm_sender, ipc_receiver) = sync_channel::<IpcEvent>(size); 
    //TODO implement mutual communication to receive current state of windowmanager

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve(ipc_sender)).unwrap();
    });

    loop {
        let event = manager.connection.borrow_mut().poll_for_event().unwrap();
        match event {
            Some(event) => manager.handle_event(&event),
            None => (),
        }
        //get_cursor_position(&manager);


        let ipc_event = wm_receiver.try_recv();
        match ipc_event {
            Ok(event) => manager.handle_ipc_event(event),
            Err(_) => (),
        }
    }
}
