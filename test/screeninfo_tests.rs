use crate::{
    config::Config, screeninfo::ScreenInfo, workspace::workspace_navigation::WorkspaceNavigation,
};
use oxide::workspace::Workspace;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::{cell::RefCell, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::Screen;
use x11rb::rust_connection::RustConnection;

struct Setup {
    pub connection: Arc<RustConnection>,
    pub screen_ref: Rc<RefCell<Screen>>,
    pub config: Rc<RefCell<Config>>,
    pub wm_state_change: Arc<(Mutex<bool>, Condvar)>,
    pub width: u32,
    pub height: u32,
}

impl Setup {
    fn new() -> Self {
        let width = 10;
        let height = 10;

        let config = Rc::new(RefCell::new(Config::new(
            "./test/test_files/config.yml".into(),
        )));
        let connection = Arc::new(RustConnection::connect(None).unwrap().0);
        let wm_state_change = Arc::new((Mutex::new(false), Condvar::new()));
        let screen_ref = Rc::new(RefCell::new(connection.setup().roots[0].clone()));

        Self {
            connection,
            screen_ref,
            config,
            wm_state_change,
            width,
            height,
        }
    }
}

pub fn get_screeninfo() -> ScreenInfo {
    let setup = Setup::new();
    let target_workspace = 1;

    ScreenInfo::new(
        setup.connection,
        setup.screen_ref,
        setup.config,
        setup.width,
        setup.height,
        setup.wm_state_change,
    )
}

#[test]
fn move_to_workspace_zero() {
    let target_workspace = 1;

    let mut screeninfo = get_screeninfo();

    screeninfo
        .go_to_workspace(WorkspaceNavigation::Number(target_workspace))
        .unwrap();
    let active_workspace_nr = screeninfo.get_active_workspace().unwrap().name;

    assert_eq!(target_workspace, active_workspace_nr);
}

#[test]
fn move_to_workspace_max_value() {
    let target_workspace = u16::max_value();

    let mut screeninfo = get_screeninfo();

    screeninfo
        .go_to_workspace(WorkspaceNavigation::Number(target_workspace))
        .unwrap();
    let active_workspace_nr = screeninfo.get_active_workspace().unwrap().name;

    assert_eq!(target_workspace, active_workspace_nr);
}

#[test]
fn test_get_next_free_workspace_nr() {
    let test_cases = vec![
        (3, 1, vec![1, 2, 4, 5, 6]),
        (4, 2, vec![1, 2, 3, 5, 6]),
        (7, 2, vec![1, 2, 3, 4, 5, 6]),
        (3, 4, vec![1, 2, 4, 5, 6]),
        (2, 1, vec![1]),
    ];

    for (expected, active_workspace, existing_workspaces) in test_cases {
        let mut screeninfo = get_screeninfo();
        screeninfo.set_test_workspaces(existing_workspaces.clone());
        screeninfo.set_test_active_workspace(active_workspace as u16);

        let next_free_workspace = screeninfo.find_next_free_workspace();
        assert_eq!(
            expected, next_free_workspace,
            "present workspaces {:?}, active workspace {}, expected {} but was {}",
            existing_workspaces, active_workspace, expected, next_free_workspace
        );
    }
}
