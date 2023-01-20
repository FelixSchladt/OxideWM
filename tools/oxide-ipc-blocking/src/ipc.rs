use zbus::{Connection, Result, dbus_proxy};
use oxide::eventhandler::events::WmActionEvent;

#[dbus_proxy(
    interface = "org.oxide.interface",
    default_service = "org.oxide.interface",
    default_path = "/org/oxide/interface"
)]
trait WmInterface {
    async fn get_status(&self) -> Result<String>;
    async fn sent_event(&self, event: WmActionEvent) -> Result<()>;
}

async fn get_proxy() -> Result<WmInterfaceProxy<'static>>{
    let connection = Connection::session().await?;
    Ok(WmInterfaceProxy::new(&connection).await?)
}

pub async fn get_state_async() -> Result<String> {
    let proxy = get_proxy().await?;
    let state = proxy.get_status().await?;
    Ok(state)
}

pub async fn sent_event_async(event: WmActionEvent) -> Result<()> {
    let proxy = get_proxy().await?;
    proxy.sent_event(event).await?;
    Ok(())
}
