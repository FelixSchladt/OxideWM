use zbus::{Connection, Result, dbus_proxy};

use clap::Parser;

use oxidewm::eventhandler::events::WmActionEvent;

#[dbus_proxy(
    interface = "org.oxide.interface",
    default_service = "org.oxide.interface",
    default_path = "/org/oxide/interface"
)]
trait WmInterface {
    async fn get_status(&self) -> Result<String>;
    async fn sent_event(&self, event: WmActionEvent) -> Result<()>;
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   command: String,

   #[arg(short, long, default_value = None)]
   args: Option<String>,
}



// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = WmInterfaceProxy::new(&connection).await?;
    if args.command == "state"  {
        let state = proxy.get_status().await?;
        println!("{}", state);
    } else {
        let ipc_command = WmActionEvent::new(args.command.as_str(), args.args);
        proxy.sent_event(ipc_command).await?;
    }


    Ok(())
}
