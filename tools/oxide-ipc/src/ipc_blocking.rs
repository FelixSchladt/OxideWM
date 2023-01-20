use zbus::{Connection, Result, dbus_proxy};
use oxide::eventhandler::events::WmActionEvent;

#[dbus_proxy(
    interface = "org.oxide.interface_blocking",
    default_service = "org.oxide.interface_blocking",
    default_path = "/org/oxide/interface_blocking",
)]
trait WmInterface {
    async fn wait_for_state_change(&self) -> Result<String>;
}


async fn get_proxy() -> Result<WmInterfaceProxy<'static>>{
    let connection = Connection::session().await?;
    Ok(WmInterfaceProxy::new(&connection).await?)
}


pub async fn wait_for_state_change_async() -> Result<String> {
    let proxy = get_proxy().await?;
    let state = proxy.wait_for_state_change().await?;
    Ok(state)
}
