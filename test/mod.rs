use crate::{
    config::Config, eventhandler::EventHandler, keybindings::KeyBindings, setup,
    windowmanager::WindowManager,
};
use std::sync::{Arc, Condvar, Mutex};
use std::{cell::RefCell, rc::Rc};

#[test]
fn setup_tests() {
    let config = Rc::new(RefCell::new(Config::new(
        "./test/test_files/config.yml".into(),
    )));
    let keybindings = KeyBindings::new(&config.borrow());

    let connection = setup::connection::get_connection(&keybindings.clone());
    let wm_state_change = Arc::new((Mutex::new(false), Condvar::new()));

    let mut manager =
        WindowManager::new(connection.clone(), config.clone(), wm_state_change.clone());

    let binding = keybindings.clone();
    let eventhandler = EventHandler::new(&mut manager, &binding);
}

// Leave these modules down here so they are ran AFTER the setup method!
pub mod config_tests;
