use crate::{
    config::Config, screeninfo::ScreenInfo, workspace::workspace_navigation::WorkspaceNavigation,
};
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::Screen;
use x11rb::rust_connection::RustConnection;

struct Setup {
    pub connection: Arc<RustConnection>,
    pub screen_ref: Rc<RefCell<Screen>>,
    pub config: Rc<RefCell<Config>>,
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
        let screen_ref = Rc::new(RefCell::new(connection.setup().roots[0].clone()));

        Self {
            connection,
            screen_ref,
            config,
            width,
            height,
        }
    }
}

pub fn get_screeninfo() -> ScreenInfo {
    let setup = Setup::new();

    ScreenInfo::new(
        setup.connection,
        setup.screen_ref,
        setup.config,
        setup.width,
        setup.height,
    )
}

#[test]
fn move_to_workspace_zero() {
    if super::in_pipeline() {
        return;
    }

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
    if super::in_pipeline() {
        return;
    }

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
    if super::in_pipeline() {
        return;
    }

    let test_cases = vec![
        (3, vec![1, 2, 4, 5, 6]),
        (4, vec![1, 2, 3, 5, 6]),
        (7, vec![1, 2, 3, 4, 5, 6]),
        (3, vec![1, 2, 4, 5, 6]),
        (2, vec![1]),
        (1, vec![5, 8, 9]),
    ];

    for (expected, existing_workspaces) in test_cases {
        let mut screeninfo = get_screeninfo();
        screeninfo.set_test_workspaces(existing_workspaces.clone());

        let next_free_workspace = screeninfo.find_next_free_workspace();
        assert_eq!(
            expected, next_free_workspace,
            "present workspaces {:?}, expected {} but was {}",
            existing_workspaces, expected, next_free_workspace
        );
    }
}
