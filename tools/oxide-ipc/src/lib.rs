mod ipc;
pub mod state;


use std::collections::HashMap;

use ipc::{get_state_async, sent_event_async};
use state::*;
use oxide::eventhandler::events::WmActionEvent;


pub fn get_state() -> String {
    async_std::task::block_on(get_state_async()).unwrap()
}

pub fn sent_event(command: &str, args: Option<String>) {
    let event= WmActionEvent::new(command, args);
    async_std::task::block_on(sent_event_async(event)).unwrap();
}

pub fn get_state_struct() -> OxideState {
    match serde_json::from_str(&get_state()){
        Ok(state) => state,
        Err(error)=>{
            println!("{}",get_state());
            println!("{error}");
            OxideState{
                screeninfo: HashMap::new(),
                config: Config { cmds: [].to_vec(), exec: [].to_vec(), exec_always: [].to_vec(), border_width: 0, 
                    border_color: "".to_string(), border_focus_color: "".to_string(), gap: 0 },
                focused_screen: 2,
            }
        }
    }
}


pub fn switch_workspace(index: usize) {
    sent_event("gotoworkspace", Some(index.to_string()));
}

pub fn next_workspace() {
    sent_event("gotoworkspace", Some("next".to_string()));
}

pub fn prev_workspace() {
    sent_event("gotoworkspace", Some("prev".to_string()));
}
