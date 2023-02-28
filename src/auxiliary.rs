use log::error;
use std::process::{Command, Stdio};
use std::sync::Arc;
use x11rb::{
 protocol::xproto::ConnectionExt, rust_connection::RustConnection,
};

pub fn exec_user_command(args: &Option<String>) {
    match args {
        Some(args) => {
            let mut args = args.split_whitespace();
            let command = args.next().unwrap();
            let args = args.collect::<Vec<&str>>().join(" ");
            match if args.is_empty() {
                Command::new(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            } else {
                Command::new(command)
                    .arg(args)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            } {
                Ok(_) => (),
                Err(_) => error!("Command {} not found", command),
            }
        }
        None => error!("User command called without values"),
    }
}

pub fn atom_name(connection: &Arc<RustConnection>, id: u32) -> String {
    let reply = connection.get_atom_name(id).unwrap().reply().unwrap();
    String::from_utf8(reply.name).unwrap()
}

pub fn get_internal_atom(connection: &Arc<RustConnection>, atm: &str) -> u32 {
    return connection
        .intern_atom(false, atm.as_bytes())
        .unwrap()
        .reply()
        .unwrap()
        .atom;
}
