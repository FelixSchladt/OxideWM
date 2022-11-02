/*
 * This module is spposed to show how to use zbus to communicate with dbus.
 */

//Theres a few different keywords when using others crates/files.
//These are `mod` and `use`.
//Here, the second one is enough.
use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};

//Structs in rust work the same as they do in C.
//The only thing thats different is the syntax to asign a
//datatype to a variable.
struct EventCounter {
    count: u64
}

//`Impl` allows to define functions for structs.
//Think of working with the struct/impl-combo as you would of working with classes in
//Python.
#[dbus_interface(name = "org.zbus.EventCounter")]
impl EventCounter {
    //Simpilar to Python, `self` gives the funtion access to its structs data.
    //Note, that hereit is a pointer which also has to be declared mutable,
    //as is common in Rust.
    fn incr_event_count(&mut self) {
        self.count += 1;
    }

    //As you can see, it is not necessary to write `return <x>;` to
    //return a value.
    //However, this might cause some issues if the to-be-returned value
    //is not the last statement of a scope (Just as a return statement would).
    fn get_event_count(&mut self) -> u64 {
        self.count
    }
}

//When using rust, it is good pratice to return a `Result<T, S>`.
//This return type is used for error handling and other logic.
pub async fn start_capsulated_dbus_server() -> Result<(), Box<dyn Error>> {
    //Initialize EventCounter struct, including its functions
    let e_counter = EventCounter { count: 0 };

    //`_` will create a variable of which the value is ignored.
    //Here, it is used to create a connection with dbus and register our
    //custom event.
    //The `?` is one way of working with the previously explained `Result<T, S>`.
    //It means, that should the function beforehand return an error,
    //the program will `panic` and immediately terminate. This is similar behavior
    //to throwing an exception without catching it.
    let _ = ConnectionBuilder::session()?
            .name("org.zbus.EventCounter")?
            .serve_at("/org/zbus/EventCounter", e_counter)?
            .build()
            .await?;

    //wait forever
    pending::<()>().await;

    //This statement is a shortcut for the `Result`-type.
    //You can either return a value with Ok(<value>) if everything goes as expected,
    //or with Err(<value>) when an issue occurs.
    //Here, the value expected for `OK()` is an empty touple as can be seen in the
    //function signature. Hence, we return it here.
    Ok(())
}
