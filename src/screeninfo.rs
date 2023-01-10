use log::warn;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::MapRequestEvent;
use crate::workspace::{Workspace, self};
use std::{cell::RefCell, rc::Rc, borrow::BorrowMut};


#[derive(Debug)]
pub struct ScreenInfo {
    pub connection: Rc<RefCell<RustConnection>>,
    pub id: u32,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u32,
    pub height: u32,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, id: u32, height: u32, width: u32) -> ScreenInfo {
        let active_workspace = 0;
        let mut workspaces = Vec::new();
        workspaces.push(Workspace::new(0, connection.clone(), 0, 0, height, width));
        ScreenInfo {
            connection,
            id,
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

    pub fn set_workspace(&mut self, workspace_nr: usize){
        if self.workspaces.len() <= workspace_nr {
            warn!("Invalid active workspace {}", workspace_nr);
            return;
        }
        self.workspaces[self.active_workspace].unmap_windows();
    }

    pub fn get_workspace_count(&self)->usize{
        return self.workspaces.len();
    }
}
