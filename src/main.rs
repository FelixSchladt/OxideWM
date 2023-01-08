pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
//pub mod zbus_interface;

//use std::sync::mpsc::{channel, Sender};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;

use std::error::Error;
use std::thread;

use x11rb::connection::Connection;


use windowmanager::{WindowManager, IpcEvent, WmActionEvent};

use std::future::pending;
use zbus::{ConnectionBuilder, dbus_interface};
//use zbus::blocking::Connection;

struct WmInterface {
    status: String,
    sender: SyncSender<IpcEvent>,
}

#[dbus_interface(name = "org.oxide.interface")]
impl WmInterface {
    fn get_status(&mut self) -> String {
        return self.status.clone();
    }

    fn event(&mut self, event: WmActionEvent) {
        self.sender.send(IpcEvent::from(event)).unwrap();
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
async fn zbus_serve(sender: SyncSender<IpcEvent>) -> Result<(), Box<dyn Error>> {
    let interface = WmInterface { 
        status: "Test".to_string(), //TODO: this needs to be filled with information from the wm 
        sender: sender.clone(),
    };
    let _ = ConnectionBuilder::session()?
        .name("org.oxide.interface")?
        .serve_at("/org/oxide/interface", interface)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}





fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = WindowManager::new();

    let size: usize = 1000;
    let (ipc_sender, wm_receiver) = sync_channel::<IpcEvent>(size);
    let (wm_sender, ipc_receiver) = sync_channel::<IpcEvent>(size);

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
