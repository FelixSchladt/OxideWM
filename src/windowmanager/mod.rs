pub mod movement;

use self::movement::Movement;

use serde::Serialize;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::{cell::RefCell, rc::Rc};

use log::{debug, error, info, warn};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{protocol::xproto::*, xcb_ffi::XCBConnection};

use crate::{
    atom::Atom,
    auxiliary::exec_user_command,
    config::Config,
    eventhandler::events::EventType,
    screeninfo::ScreenInfo,
    workspace::{
        workspace_layout::WorkspaceLayout, workspace_navigation::WorkspaceNavigation, Workspace,
    },
};

#[derive(Debug, Serialize)]
pub struct WindowManagerState {
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Config,
    pub focused_screen: u32,
}

#[derive(Debug, Clone)]
pub struct WindowManager {
    pub connection: Arc<XCBConnection>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub focused_screen: u32,
    pub moved_window: Option<u32>,
    pub restart: bool,
}

impl WindowManager {
    pub fn new(
        connection: Arc<XCBConnection>,
        config: Rc<RefCell<Config>>,
        wm_state_change: Arc<(Mutex<bool>, Condvar)>,
    ) -> WindowManager {
        let screeninfo = HashMap::new();

        let focused_screen = 0;
        //TODO: Get focused screen from X11
        // Currently the screen setup last is taken as active.
        // We should discuss if this default behaviour is ok or not.

        let mut manager = WindowManager {
            connection,
            screeninfo,
            config,
            focused_screen,
            moved_window: None,
            restart: false,
        };

        manager.setup_screens(wm_state_change);
        manager.autostart_exec();
        manager.autostart_exec_always();
        let result = manager.connection.flush();
        if result.is_err() {
            info!("Failed to flush rust connection");
        }

        manager
    }

    pub fn restart_wm(&mut self, config: Rc<RefCell<Config>>) {
        self.config = config;
        self.autostart_exec_always();
        self.connection.flush().unwrap();
        self.restart = false;
    }

    fn autostart_exec(&self) {
        for command in &self.config.borrow().exec {
            info!("executing exec, command: {}", command);
            exec_user_command(&Some(command.clone()));
        }
    }

    fn autostart_exec_always(&self) {
        for command in &self.config.borrow().exec_always {
            info!("executing exec always, command: {}", command);
            exec_user_command(&Some(command.clone()));
        }
    }

    pub fn get_state(&self) -> WindowManagerState {
        WindowManagerState {
            screeninfo: self.screeninfo.clone(),
            config: self.config.borrow().clone(),
            focused_screen: self.focused_screen.clone(),
        }
    }

    pub fn run_event_proxy(connection: Arc<XCBConnection>, queue: Arc<Mutex<Sender<EventType>>>) {
        debug!("Started waiting for X-Event");

        loop {
            match connection.wait_for_event() {
                Ok(event) => {
                    debug!("Transvering X-Event into Queue {:?}", event);

                    let event_typ = EventType::X11rbEvent(event);
                    if let Err(error) = queue.lock().unwrap().send(event_typ) {
                        warn!("Could not insert event into event queue {}", error);
                    };
                }
                Err(error) => {
                    error!("Error retreiving Event from Window manager {:?}", error);
                }
            };
        }
    }

    fn get_active_workspace(&mut self) -> &mut Workspace {
        let screen_info = self.screeninfo.get_mut(&self.focused_screen).unwrap();
        screen_info.get_active_workspace().unwrap()
    }

    fn get_focused_window(&mut self) -> Option<u32> {
        let workspace = self.get_active_workspace();
        workspace.get_focused_window()
    }

    pub fn handle_keypress_focus(&mut self, args_option: Option<String>) {
        if let Some(args) = args_option {
            match Movement::try_from(args.as_str()) {
                Ok(movement) => {
                    let workspace = self.get_active_workspace();
                    workspace.move_focus(movement);
                }
                Err(_) => warn!("Could not parse movement from argument {}", args),
            }
        } else {
            warn!("Argument must be provided");
        }
    }

    pub fn handle_keypress_move(&mut self, args_option: Option<String>) {
        if let Some(args) = args_option {
            match Movement::try_from(args.as_str()) {
                Ok(movement) => {
                    let workspace = self.get_active_workspace();
                    workspace.move_window(movement);
                }
                Err(_) => warn!("Could not parse movement from argument {}", args),
            }
        } else {
            warn!("Argument must be provided");
        }
    }

    pub fn handle_keypress_kill(&mut self) {
        let focused_window = self.get_focused_window();
        debug!("Focused window: {:?}", focused_window);
        if let Some(winid) = focused_window {
            self.get_active_workspace().kill_window(&winid);
        } else {
            error!("ERROR: No window to kill \nShould only happen on an empty screen");
        }
    }

    pub fn handle_keypress_layout(&mut self, args: Option<String>) {
        let active_workspace = self.get_active_workspace();

        match args {
            Some(args) => {
                let layout = WorkspaceLayout::try_from(args.as_str());
                if layout.is_err() {
                    warn!("Layout could not be parsed from argument {}", args);
                    return;
                }
                active_workspace.set_layout(layout.unwrap());
            }
            None => active_workspace.next_layout(),
        }
    }

    pub fn handle_keypress_go_to_workspace(&mut self, args_option: Option<String>) {
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            match arg_option {
                Ok(arg) => {
                    if let Err(error) = screen.switch_workspace(arg) {
                        warn!("Could not go to workspace {}", error);
                    }
                }
                Err(error) => warn!("Could not go to workspace {}", error),
            }
            screen.state_changed();
        } else {
            warn!("Could not switch workspace, no screen was focused");
        }
    }

    pub fn handle_move_to_workspace(&mut self, args_option: Option<String>) {
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            match arg_option {
                Ok(arg) => {
                    if let Err(error) = screen.move_window_to_workspace(arg) {
                        warn!("failed to move window to workspace {}", error);
                    }
                }
                Err(error) => warn!("Could not move to workspace {}", error),
            }
            screen.state_changed();
        } else {
            warn!("Could not move to workspace, no screen was focused");
        }
    }

    pub fn handle_move_to_workspace_follow(&mut self, args_option: Option<String>) {
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            if let Ok(arg) = arg_option {
                if let Err(error) = screen.move_window_to_workspace_and_follow(arg) {
                    warn!("failed to move window to workspace and follow {}", error);
                }
            } else if let Err(error) = arg_option {
                warn!("Could not move to workspace {}", error);
            }
            screen.state_changed();
        } else {
            warn!("Could not move to workspace, no screen was focused");
        }
    }

    pub fn handle_move_to_or_create_workspace(&mut self, args_option: Option<String>) {
        let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
        match arg_option {
            Ok(arg) => {
                let screen = match self.screeninfo.get_mut(&self.focused_screen) {
                    Some(screen) => screen,
                    None => {
                        warn!("No focused screen");
                        return;
                    }
                };
                if let Err(error) = screen.move_to_or_create_workspace(arg) {
                    warn!("{error}")
                }
                screen.state_changed();
            }
            Err(error) => warn!("could not parse arguments {}", error),
        };
    }

    pub fn handle_quit_workspace(&mut self) {
        let active_workspace_name = self.get_active_workspace().name;

        if let Some(screen) = self.screeninfo.get_mut(&self.focused_screen) {
            info!("quitting workspace {}", active_workspace_name);
            if let Err(error) = screen.quit_workspace(active_workspace_name) {
                warn!("could not quit workspace {error}");
            }
            screen.state_changed();
        } else {
            warn!("No screen was focused");
        }
    }

    pub fn handle_keypress_fullscreen(&mut self) {
        self.get_active_workspace().toggle_fullscreen();
    }

    fn setup_screens(&mut self, wm_state_change: Arc<(Mutex<bool>, Condvar)>) {
        for screen in self.connection.setup().roots.iter() {
            let screen_ref = Rc::new(RefCell::new(screen.clone()));
            let screenstruct = ScreenInfo::new(
                self.connection.clone(),
                screen_ref.clone(),
                self.config.clone(),
                screen.width_in_pixels as u32,
                screen.height_in_pixels as u32,
                wm_state_change.clone(),
            );
            self.screeninfo.insert(screen.root, screenstruct);
            self.focused_screen = screen.root;
            debug!(
                "screen widht: {} screen height: {}",
                screen.width_in_pixels, screen.height_in_pixels
            );
        }
    }

    pub fn handle_event_enter_notify(&mut self, event: &EnterNotifyEvent) {
        let mut winid = event.event;
        if self.moved_window.is_some() {
            winid = self.moved_window.unwrap();
            self.moved_window = None;
        }

        let active_workspace = self.get_active_workspace();
        active_workspace.focus_window(winid);
    }

    pub fn handle_event_leave_notify(&mut self, _event: &LeaveNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.unfocus_window();
    }

    pub fn handle_event_destroy_notify(&mut self, event: &DestroyNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.remove_window(&event.window);
    }

    pub fn atom_name(&self, id: u32) -> String {
        let reply = self.connection.get_atom_name(id).unwrap().reply().unwrap();
        self.connection.flush().unwrap();
        String::from_utf8(reply.name).unwrap()
    }

    //Note to get general atoms look at
    //https://github.com/sminez/penrose/blob/develop/src/x11rb/mod.rs lines 404-500
    pub fn atom_window_type_dock(&self, winid: u32) -> bool {
        let atom_intern = self
            .connection
            .intern_atom(false, Atom::NetWmWindowType.as_ref().as_bytes())
            .unwrap()
            .reply()
            .unwrap()
            .atom;

        self.connection.flush().unwrap();
        let atom_reply = self
            .connection
            .get_property(false, winid, atom_intern, AtomEnum::ANY, 0, 1024)
            .unwrap()
            .reply();
        if let Ok(atom_reply) = atom_reply {
            self.connection.flush().unwrap();

            let prop_type = match atom_reply.type_ {
                0 => return false, // Null response
                atomid => self.atom_name(atomid),
            };

            let wm_type = Atom::NetWindowTypeDock.as_ref();
            if prop_type == "ATOM" {
                let atoms = atom_reply
                    .value32()
                    .unwrap()
                    .map(|a| self.atom_name(a))
                    .collect::<Vec<String>>();
                if atoms.contains(&wm_type.to_string()) {
                    info!("Spawned window is of type _NET_WM_WINDOW_TYPE_DOCK");
                    return true;
                }
            }
        }
        false
    }

    pub fn handle_create_notify(&mut self, event: &CreateNotifyEvent) {
        if self.atom_window_type_dock(event.window) {
            self.screeninfo
                .get_mut(&event.parent.clone())
                .unwrap()
                .add_status_bar(event);
        }
    }

    pub fn handle_map_request(&mut self, event: &MapRequestEvent) {
        if !self.atom_window_type_dock(event.window.clone()) {
            self.screeninfo
                .get_mut(&event.parent.clone())
                .unwrap()
                .on_map_request(event);
        }
    }
}
