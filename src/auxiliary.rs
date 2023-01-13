use std::process::{Command, Stdio, exit};
use log::error;

pub fn exec_user_command(args: &Option<String>) {
    match args {
        Some(args) => {
            let mut args = args.split_whitespace();
            let command = args.next().expect("User command execution failed!");
            let args = args.collect::<Vec<&str>>().join(" ");
            if args.is_empty() {
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
            }.expect("Failed to execute user command: Does the program exist?");
        },
        None => {
            error!("User command called without values");
            exit(-1);
        },
    }
}
