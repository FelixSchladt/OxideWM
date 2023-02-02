use crate::{config::Config, screeninfo::ScreenInfo};
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

#[test]
fn crate_new_workspace() {
    let setup = Setup::new();

    ScreenInfo::new(
        setup.connection,
        setup.screen_ref,
        setup.config,
        setup.width,
        setup.height,
        setup.wm_state_change,
    );

    //TODO continue here
}
