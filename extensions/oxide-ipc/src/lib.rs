mod ipc;

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::ipc::state_signal_channel_async;
use ipc::{get_state_async, sent_event_async};
use oxide_common::ipc::{action_event::WmActionEvent, state::OxideStateDto};

pub fn get_state() -> String {
    async_std::task::block_on(get_state_async()).unwrap()
}

pub fn sent_event(command: &str, args: Option<String>) ->  Result<(), String>{
    let event = match WmActionEvent::new(command, args) {
        Ok(event) => event,
        Err(msg)  => {
            return Err(msg);
        }
    };

    async_std::task::block_on(sent_event_async(event)).ok();
    Ok(())
}

pub fn get_state_struct() -> OxideStateDto {
    match serde_json::from_str(&get_state()) {
        Ok(state) => state,
        Err(e) => panic!("Error parsing state: {}", e),
    }
}

pub fn state_signal_channel(sender: Arc<Mutex<Sender<OxideStateDto>>>) {
    println!("Waiting for state change");
    async_std::task::block_on(state_signal_channel_async(sender)).unwrap();
}

pub fn switch_workspace(index: usize) {
    sent_event("gotoworkspace", Some(index.to_string())).ok();
}

pub fn next_workspace() {
    sent_event("gotoworkspace", Some("next".to_string())).ok();
}

pub fn prev_workspace() {
    sent_event("gotoworkspace", Some("prev".to_string())).ok();
}
