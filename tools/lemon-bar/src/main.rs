use oxideipc;
use oxideipc::state::*;
use colored::*;
use itertools::Itertools;


fn print_ws() {
    let state = oxideipc::get_state_struct();
    let screen = state.focused_screen;
    let workspaces = state.get_workspaces(screen);
    let active_workspace = state.screeninfo[&screen].active_workspace;

    let workspaces_sorted = workspaces.iter().sorted_by_key(|w| w.0);

    let mut i = 0;
    let len = workspaces.len();
    for (id, workspace) in workspaces_sorted {
        i += 1;
        if *id as usize == active_workspace {
            let ws_str = format!("{}", workspace.name).bold().blue();
            print!("{}", ws_str);
        } else {
            let ws_str = format!("{}", workspace.name);
            print!("{}", ws_str);
        }
        if i < len {
            print!(" | ");
        } else {
            println!(" ");
        }
    }
}

fn main() {
    loop {
        print_ws();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

