use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::error::Error;

use crate::windowmanager::{IpcEvent, WmActionEvent};


use std::future::pending;
use zbus::{ConnectionBuilder, dbus_interface};

struct WmInterface {
    sender: Arc<Mutex<Sender<IpcEvent>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

#[dbus_interface(name = "org.oxide.interface")]
impl WmInterface {
    fn get_status(&mut self) -> String {
        self.sender.lock().unwrap().send(IpcEvent { status: true, event: None }).unwrap();
        self.receiver.lock().unwrap().recv().unwrap()
    }

    fn event(&mut self, event: WmActionEvent) {
        self.sender.lock().unwrap().send(IpcEvent::from(event)).unwrap();
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
pub async fn zbus_serve(sender: Arc<Mutex<Sender<IpcEvent>>>, receiver: Arc<Mutex<Receiver<String>>>) -> Result<(), Box<dyn Error>> {
    let interface = WmInterface { 
        sender: sender,
        receiver: receiver,
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
