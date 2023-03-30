use clap::Parser;
use oxideipc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: String,
    args: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.command == "state" {
        let state = oxideipc::get_state();
        println!("{}", state);
    } else {
        match oxideipc::sent_event(args.command.as_str(), args.args) {
            Ok(())   => {},
            Err(_) => {
                println!("Error: Invalid command! Run 'oxide-msg --help' to view usage.");
            }
        };
    }
    Ok(())
}
