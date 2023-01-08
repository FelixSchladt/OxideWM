use zbus::{Connection, Result, dbus_proxy};

use oxidewm::config::WmCommands;
use oxidewm::windowmanager::WmActionEvent;

#[dbus_proxy(
    interface = "org.oxide.interface",
    default_service = "org.oxide.interface",
    default_path = "/org/oxide/interface"
)]
trait WmInterface {
    async fn get_status(&self) -> Result<String>;
    async fn event(&self, event: WmActionEvent) -> Result<()>;
}


// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = WmInterfaceProxy::new(&connection).await?;
    let reply = proxy.get_status().await?;
    println!("{reply}");

    let ipc_command = WmActionEvent::new("exec", Some("kitty".to_string()));

    proxy.event(ipc_command).await?;

    Ok(())
}
