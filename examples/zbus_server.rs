use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};
use zbus::{ObjectServer, SignalContext, MessageHeader};

use std::time::Duration;

#[derive(Debug, Clone)]
struct Greeter {
    count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }

    #[dbus_interface(signal)]
    async fn test_signal(sig_cnt: &SignalContext<'_>, count: u64) -> zbus::Result<()> {}
}

// Although we use `async-std` here, you can use any async runtime of choice.
async fn test() -> Result<(), Box<dyn Error>> {
    let mut greeter = Greeter { count: 0 };
    let zbus_connection = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter.clone())?
        .build()
        .await?;

    // Do other things or go to wait forever
    let path = "/org/zbus/MyGreeter".to_string();
    loop {
        std::thread::sleep(Duration::from_secs(1));
        let signal_cntx = SignalContext::new(&zbus_connection, path.as_str()).unwrap();
        Greeter::test_signal(&signal_cntx, greeter.count.clone()).await?;
        greeter.count += 1;
    }


    Ok(())
}

/*
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test().await

}*/

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(test())
}
