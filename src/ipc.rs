use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};

use log::warn;
use oxide_common::ipc::action_event::WmActionEvent;

use std::error::Error;

use crate::eventhandler::events::{EventType, IpcEvent};

use lazy_static::lazy_static;
use zbus::{dbus_interface, ConnectionBuilder, SignalContext};

lazy_static! {
    static ref STATE_CHANGE: Arc<(Mutex<bool>, Condvar)> =
        Arc::new((Mutex::new(false), Condvar::new()));
}

pub fn signal_state_change() {
    let (lock, cvar) = &**STATE_CHANGE;
    let mut state_changed = lock.lock().unwrap();
    *state_changed = true;
    // We notify the condvar that the value has changed.
    cvar.notify_one();
}

struct WmInterface {
    event_send_channel: Arc<Mutex<Sender<EventType>>>,
    status_receive_channel: Arc<Mutex<Receiver<String>>>,
}

#[dbus_interface(name = "org.oxide.interface")]
impl WmInterface {
    fn get_status(&mut self) -> String {
        let event = EventType::OxideEvent(IpcEvent {
            status: true,
            event: None,
        });

        //flushing channel
        while let Ok(_) = self.status_receive_channel.lock().unwrap().try_recv() {
            warn!("There occured a flush of an old state: If this happens often, please open an issue on github");
        }
        //send state request to wm manager via channel
        self.event_send_channel.lock().unwrap().send(event).unwrap();
        //block om receiving channel until state has been sent by the wm
        self.status_receive_channel.lock().unwrap().recv().unwrap()
    }

    fn sent_event(&mut self, event: WmActionEvent) {
        let event = EventType::OxideEvent(IpcEvent {
            status: false,
            event: Some(event),
        });
        //sent event to wm manager via channel
        self.event_send_channel.lock().unwrap().send(event).unwrap();
    }

    #[dbus_interface(signal)]
    async fn state_change(sig_cnt: &SignalContext<'_>, state: String) -> zbus::Result<()> {}
}

pub async fn zbus_serve(
    event_send_channel: Arc<Mutex<Sender<EventType>>>,
    status_receive_channel: Arc<Mutex<Receiver<String>>>,
) -> Result<(), Box<dyn Error>> {
    let event_send_clone = event_send_channel.clone();
    let status_receive_clone = status_receive_channel.clone();
    let interface = WmInterface {
        event_send_channel: event_send_clone,
        status_receive_channel: status_receive_clone,
    };

    let path = "/org/oxide/interface";

    let zbus_connection = ConnectionBuilder::session()?
        .name("org.oxide.interface")?
        .serve_at(path, interface)?
        .build()
        .await?;

    loop {
        let (lock, cvar) = &**STATE_CHANGE;
        let mut changed = lock.lock().unwrap();

        while !*changed {
            changed = cvar.wait(changed).unwrap();
        }
        *changed = false;

        //flushing channel
        while let Ok(_) = status_receive_channel.lock().unwrap().try_recv() {
            warn!("There occured a flush of an old state: If this happens often, please open an issue on github");
        }

        log::info!("state change signal");
        event_send_channel
            .lock()
            .unwrap()
            .send(EventType::OxideEvent(IpcEvent {
                status: true,
                event: None,
            }))?;
        let state = status_receive_channel.lock().unwrap().recv()?;

        let signal_cntx = SignalContext::new(&zbus_connection, path)?;
        WmInterface::state_change(&signal_cntx, state).await?;
    }
}
