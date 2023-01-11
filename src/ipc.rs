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
        //send state request to wm manager via channel
        self.sender.lock().unwrap().send(IpcEvent { status: true, event: None }).unwrap();
        //block om receiving channel until state has been sent by the wm
        self.receiver.lock().unwrap().recv().unwrap()
    }

    fn sent_event(&mut self, event: WmActionEvent) {
        //sent event to wm manager via channel
        self.sender.lock().unwrap().send(IpcEvent::from(event)).unwrap();
    }
}

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

    pending::<()>().await;

    Ok(())
}
