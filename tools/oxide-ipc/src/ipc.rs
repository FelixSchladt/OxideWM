use async_std::stream::StreamExt;
use zbus::{dbus_proxy, Connection, Result};

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::state::OxideState;
use crate::events::*;

#[dbus_proxy(
    interface = "org.oxide.interface",
    default_service = "org.oxide.interface",
    default_path = "/org/oxide/interface"
)]
trait WmInterface {
    async fn get_status(&self) -> Result<String>;
    async fn sent_event(&self, event: WmActionEvent) -> Result<()>;
    //async fn wait_for_state_change(&self) -> Result<String>;
    #[dbus_proxy(signal)]
    async fn state_change(&self, state: String) -> Result<()>;
}
async fn get_proxy() -> Result<WmInterfaceProxy<'static>> {
    let connection = Connection::session().await?;
    Ok(WmInterfaceProxy::new(&connection).await?)
}

/*
pub async fn wait_for_state_change_async() -> Result<String> {
    let proxy = get_proxy().await?;
    println!("Waiting for state change 4");
    let mut reply = proxy.receive_state_change().await?;
    let rep = reply.next().await.unwrap();
    println!("Got state change: {:?}", rep);
    Ok(rep.args()?.state)
}*/

/*
pub async fn state_signal_stream() -> Result<zbus::fdo::ResultStream<'static, String>> {
    let proxy = get_proxy().await?;
    let reply = proxy.receive_get_state().await?;
    Ok(reply)
}
*/

pub async fn state_signal_channel_async(sender: Arc<Mutex<Sender<OxideState>>>) -> Result<()> {
    let proxy = get_proxy().await?;
    loop {
        let mut reply = proxy.receive_state_change().await?;
        let rep = reply.next().await.unwrap();
        let state = serde_json::from_str(&rep.args()?.state).unwrap();
        sender.lock().unwrap().send(state).unwrap();
    }
}

pub async fn get_state_async() -> Result<String> {
    let proxy = get_proxy().await?;
    let state = proxy.get_status().await?;
    Ok(state)
}

/*
pub async fn wait_for_state_change_async() -> Result<String> {
    let proxy = get_proxy().await?;
    let state = proxy.wait_for_state_change().await?;
    Ok(state)
}*/

pub async fn sent_event_async(event: WmActionEvent) -> Result<()> {
    let proxy = get_proxy().await?;
    proxy.sent_event(event).await?;
    Ok(())
}
