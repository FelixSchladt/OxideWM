use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};

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
}

// Although we use `async-std` here, you can use any async runtime of choice.
async fn test() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let _ = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

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
