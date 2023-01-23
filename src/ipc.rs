use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::error::Error;

use crate::eventhandler::events::{IpcEvent, WmActionEvent,EnumEventType};

use zbus::{ConnectionBuilder, dbus_interface};

struct WmInterface {
    event_send_channel: Arc<Mutex<Sender<EnumEventType>>>,
    status_receive_channel: Arc<Mutex<Receiver<String>>>,
}

#[dbus_interface(name = "org.oxide.interface")]
impl WmInterface {
    fn get_status(&mut self) -> String {
        let event = EnumEventType::OxideEvent(IpcEvent { status: true, event: None });
        //send state request to wm manager via channel
        self.event_send_channel.lock().unwrap().send(event).unwrap();
        //block om receiving channel until state has been sent by the wm
        self.status_receive_channel.lock().unwrap().recv().unwrap()
    }

    fn sent_event(&mut self, event: WmActionEvent) {
        let event = EnumEventType::OxideEvent(IpcEvent { status: true, event: Some(event) });
        //sent event to wm manager via channel
        self.event_send_channel.lock().unwrap().send(event).unwrap();
    }
}

pub async fn zbus_serve(
    event_send_channel: Arc<Mutex<Sender<EnumEventType>>>,
    status_receive_channel: Arc<Mutex<Receiver<String>>>
) -> Result<(), Box<dyn Error>> {

    let interface = WmInterface {
        event_send_channel,
        status_receive_channel,
    };

    ConnectionBuilder::session()?
                      .name("org.oxide.interface")?
                      .serve_at("/org/oxide/interface", interface)?
                      .build()
                      .await?;

    Ok(())
}
