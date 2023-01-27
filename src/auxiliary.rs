use log::error;
use std::process::{Command, Stdio};

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
