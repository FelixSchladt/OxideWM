use std::sync::mpsc::SyncSender;

use std::error::Error;

use crate::windowmanager::{IpcEvent, WmActionEvent};

use std::future::pending;
use zbus::{ConnectionBuilder, dbus_interface};

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
pub async fn zbus_serve(sender: SyncSender<IpcEvent>) -> Result<(), Box<dyn Error>> {
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
