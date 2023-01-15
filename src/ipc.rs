use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::error::Error;

use crate::eventhandler::events::{IpcEvent, WmActionEvent};


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

/// # Errors
///
/// Errors might occur when the `ConnectionBuilder` cannot establish a session to
/// `org.oxide.interface`
pub async fn zbus_serve(sender: Arc<Mutex<Sender<IpcEvent>>>, receiver: Arc<Mutex<Receiver<String>>>) -> Result<(), Box<dyn Error>> {
    let interface = WmInterface {
        sender,
        receiver,
    };

    ConnectionBuilder::session()?
                      .name("org.oxide.interface")?
                      .serve_at("/org/oxide/interface", interface)?
                      .build()
                      .await?;

    Ok(())
}
