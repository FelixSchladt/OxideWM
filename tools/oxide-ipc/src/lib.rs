mod ipc;
pub mod state;


use ipc::{get_state_async, sent_event_async};
use state::*;
use oxidewm::eventhandler::events::WmActionEvent;


pub fn get_state() -> String {
    async_std::task::block_on(get_state_async()).unwrap()
}

pub fn sent_event(command: &str, args: Option<String>) {
    let event= WmActionEvent::new(command, args);
    async_std::task::block_on(sent_event_async(event)).unwrap();
}

pub fn get_state_struct() -> OxideState {
    serde_json::from_str(&get_state()).unwrap()
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
