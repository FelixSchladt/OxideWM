/*
 * Export your source code into as different file and only use this main
 * function to call a custom init finction.
 * This way, multiple things can be tested with the same main file.
 */

use std::error::Error;

//IMPORTANT: Comment out unused `mod` statements to avoid lengthy build times
//mod gdb_practice;
mod ipc_sample;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //gdb_practice::start_gdb_practice();
    ipc_sample::start_capsulated_dbus_server().await
}
