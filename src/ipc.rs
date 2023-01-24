use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, Condvar};

use std::error::Error;

use crate::eventhandler::events::{IpcEvent, WmActionEvent};


use zbus::{ConnectionBuilder, dbus_interface, SignalContext};

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

    #[dbus_interface(signal)]
    async fn state_change(sig_cnt: &SignalContext<'_>, state: String) -> zbus::Result<()> {}
}

pub async fn zbus_serve(sender: Arc<Mutex<Sender<IpcEvent>>>, 
                        receiver: Arc<Mutex<Receiver<String>>>,
                        wm_state_change: Arc<(Mutex<bool>, Condvar)>,
                        ) -> Result<(), Box<dyn Error>> {
    let interface = WmInterface {
        sender: sender.clone(),
        receiver: receiver.clone(),
    };
    
    let path = "/org/oxide/interface";

    let zbus_connection = ConnectionBuilder::session()?
                      .name("org.oxide.interface")?
                      .serve_at(path, interface)?
                      .build()
                      .await?;
    
    loop {
        let (lock, cvar) = &*wm_state_change;
        let mut changed = lock.lock().unwrap();
        
        while !*changed {
            changed = cvar.wait(changed).unwrap();
        }
        *changed = false;

        log::info!("state change signal");
        sender.lock().unwrap().send(IpcEvent {status: true, event: None})?;
        let state = receiver.lock().unwrap().recv()?;
        

        let signal_cntx = SignalContext::new(&zbus_connection, path)?;
        WmInterface::state_change(&signal_cntx, state).await?;
    }
}
