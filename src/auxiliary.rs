use std::process::{Command, Stdio};

pub fn exec_user_command(args: &Option<String>) {
    match args {
        Some(args) => {
            let mut args = args.split_whitespace();
            let command = args.next().unwrap();
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
            }.unwrap();
        },
        None => panic!("User command called without args"),
    }
}
