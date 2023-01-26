use zbus::{Connection, Result, dbus_proxy};
use async_std::stream::StreamExt;

#[dbus_proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
    #[dbus_proxy(signal)]
    async fn test_signal(&self, counter: u64) -> Result<()>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()>  {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(&connection).await?;
    //let reply = proxy.say_hello("Maria").await?;
    //println!("{reply}");
    loop {
        let mut reply = proxy.receive_test_signal().await?;
        let rep = reply.next().await.unwrap();
        println!("{:?}", rep);
        println!("{:?}", rep.args());
    }
}
