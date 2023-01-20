//mod ipc;
mod ipc_blocking;
pub mod state;

use ipc_blocking::wait_for_state_change_async;
use state::*;
use oxide::eventhandler::events::WmActionEvent;


pub fn wait_for_state_change() -> OxideState {
    let state = async_std::task::block_on(wait_for_state_change_async()).unwrap();
    serde_json::from_str(&state).unwrap()
}
