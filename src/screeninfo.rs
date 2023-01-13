use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use serde::Serialize;

use crate::workspace::Workspace;
use std::{cell::RefCell, rc::Rc};


#[derive(Debug, Clone, Serialize)]
pub struct ScreenInfo {
    #[serde(skip_serializing)]
    _connection: Rc<RefCell<RustConnection>>,
    #[serde(skip_serializing)]
    _screen_ref: Rc<RefCell<Screen>>,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u16,
    pub height: u16,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, screen_ref: Rc<RefCell<Screen>>, height: u16, width: u16) -> ScreenInfo {
        let active_workspace = 0;
        let workspaces = Vec::new();
        ScreenInfo {
            _connection: connection,
            _screen_ref: screen_ref,
            workspaces,
            active_workspace,
            width,
            height,
        }
    }

    pub fn on_map_request(&mut self, event: &MapRequestEvent) {
        println!("WINMAN: MapRequestEvent: {:?}", event);
        let workspace = &mut self.workspaces[self.active_workspace.clone()];
        workspace.new_window(event.window);
        workspace.remap_windows();
    }
}
