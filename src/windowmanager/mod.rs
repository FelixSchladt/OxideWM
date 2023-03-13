pub mod movement;

use self::movement::Movement;

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, rc::Rc};

use log::{debug, error, info, warn};
use oxide_common::ipc::state::OxideStateDto;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{protocol::xproto::*, rust_connection::RustConnection};

use crate::{
    atom::Atom,
    auxiliary::{atom_name, exec_user_command, get_internal_atom},
    config::Config,
    eventhandler::events::EventType,
    ipc::signal_state_change,
    screeninfo::ScreenInfo,
    workspace::{
        workspace_layout::WorkspaceLayout, workspace_navigation::WorkspaceNavigation, Workspace,
    },
};

#[derive(Debug, Clone)]
pub struct WindowManager {
    pub connection: Arc<RustConnection>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub focused_screen: u32,
    pub moved_window: Option<u32>,
    pub restart: bool,
}

impl WindowManager {
    pub fn new(connection: Arc<RustConnection>, config: Rc<RefCell<Config>>) -> WindowManager {
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

        manager.setup_screens();
        manager.autostart_exec();
        manager.autostart_exec_always();
        let result = manager.connection.flush();
        if result.is_err() {
            info!("failed to flush rust connection");
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

    pub fn get_state(&self) -> OxideStateDto {
        let screen_info = self
            .screeninfo
            .iter()
            .map(|(key, info)| (*key, info.to_dto()))
            .collect();
        OxideStateDto {
            screeninfo: screen_info,
            config: self.config.borrow().to_dto(),
            focused_screen: self.focused_screen.clone(),
        }
    }

    pub fn run_event_proxy(connection: Arc<RustConnection>, queue: Arc<Mutex<Sender<EventType>>>) {
        debug!("started waiting for X-Event");

        loop {
            match connection.wait_for_event() {
                Ok(event) => {
                    debug!("transferring X-Event into queue {:?}", event);

                    let event_typ = EventType::X11rbEvent(event);
                    if let Err(error) = queue.lock().unwrap().send(event_typ) {
                        warn!("could not insert event into event queue {}", error);
                    };
                }
                Err(error) => {
                    error!("error retreiving event from window manager {:?}", error);
                }
            };
        }
    }

    fn get_active_workspace(&mut self) -> Rc<RefCell<Workspace>> {
        let screen_info = self.screeninfo.get_mut(&self.focused_screen).unwrap();
        screen_info.get_active_workspace()
    }

    fn get_focused_window(&mut self) -> Option<u32> {
        self.get_active_workspace().borrow().get_focused_window()
    }

    pub fn handle_keypress_focus(&mut self, args_option: Option<String>) {
        if let Some(args) = args_option {
            match Movement::try_from(args.as_str()) {
                Ok(movement) => {
                    let workspace = self.get_active_workspace();
                    workspace.borrow_mut().move_focus(movement);
                }
                Err(_) => warn!("could not parse movement from argument {}", args),
            }
        } else {
            warn!("argument must be provided");
        }
    }

    pub fn handle_keypress_move(&mut self, args_option: Option<String>) {
        if let Some(args) = args_option {
            match Movement::try_from(args.as_str()) {
                Ok(movement) => {
                    let workspace = self.get_active_workspace();
                    workspace.borrow_mut().move_window(movement);
                }
                Err(_) => warn!("could not parse movement from argument {}", args),
            }
        } else {
            warn!("argument must be provided");
        }
    }

    pub fn handle_keypress_kill(&mut self) {
        let focused_window = self.get_focused_window();
        debug!("focused window: {:?}", focused_window);
        if let Some(winid) = focused_window {
            self.get_active_workspace().borrow_mut().kill_window(&winid);
        } else {
            error!("ERROR: no window to kill \nshould only happen on an empty screen");
        }
    }

    pub fn handle_keypress_layout(&mut self, args: Option<String>) {
        let active_workspace = self.get_active_workspace();

        match args {
            Some(args) => {
                let layout = WorkspaceLayout::try_from(args.as_str());
                if layout.is_err() {
                    warn!("layout could not be parsed from argument {}", args);
                    return;
                }
                active_workspace.borrow_mut().set_layout(layout.unwrap());
            }
            None => active_workspace.borrow_mut().next_layout(),
        }
    }

    pub fn handle_keypress_go_to_workspace(&mut self, args_option: Option<String>) {
        debug!("handeling keypress go to workspace");
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            match arg_option {
                Ok(arg) => {
                    if let Err(error) = screen.go_to_workspace(arg) {
                        warn!("could not go to workspace {}", error);
                    }
                }
                Err(error) => warn!("could not go to workspace {}", error),
            }
            signal_state_change();
        } else {
            warn!("could not switch workspace, no screen was focused");
        }
    }

    pub fn handle_move_to_workspace(&mut self, args_option: Option<String>) {
        debug!("handeling keypress move to workspace");
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            match arg_option {
                Ok(arg) => {
                    if let Err(error) = screen.move_window_to_workspace(arg) {
                        warn!("failed to move window to workspace {}", error);
                    }
                }
                Err(error) => warn!("could not move to workspace {}", error),
            }
            signal_state_change();
        } else {
            warn!("could not move to workspace, no screen was focused");
        }
    }

    pub fn handle_move_to_workspace_follow(&mut self, args_option: Option<String>) {
        debug!("handeling keypress move to workspace and follow");
        let screen_option = self.screeninfo.get_mut(&self.focused_screen);
        if let Some(screen) = screen_option {
            let arg_option = WorkspaceNavigation::parse_workspace_navigation(args_option);
            if let Ok(arg) = arg_option {
                if let Err(error) = screen.move_window_to_workspace_and_follow(arg) {
                    warn!("failed to move window to workspace and follow {}", error);
                }
            } else if let Err(error) = arg_option {
                warn!("could not move to workspace {}", error);
            }
            signal_state_change();
        } else {
            warn!("could not move to workspace, no screen was focused");
        }
    }

    pub fn handle_quit_workspace(&mut self) {
        debug!("handeling keypress quit workspace");

        if let Some(screen) = self.screeninfo.get_mut(&self.focused_screen) {
            if let Err(error) = screen.quit_workspace_select_new() {
                warn!("could not quit workspace {error}");
            }
            signal_state_change();
        } else {
            warn!("no screen was focused");
        }
    }

    pub fn handle_keypress_fullscreen(&mut self) {
        self.get_active_workspace().borrow_mut().toggle_fullscreen();
    }

    fn setup_screens(&mut self) {
        for screen in self.connection.setup().roots.iter() {
            let screen_ref = Rc::new(RefCell::new(screen.clone()));
            let screenstruct = ScreenInfo::new(
                self.connection.clone(),
                screen_ref.clone(),
                self.config.clone(),
                screen.width_in_pixels as u32,
                screen.height_in_pixels as u32,
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
        active_workspace.borrow_mut().focus_window(winid);
    }

    pub fn handle_event_leave_notify(&mut self, _event: &LeaveNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.borrow_mut().unfocus_window();
    }

    pub fn handle_event_destroy_notify(&mut self, event: &DestroyNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.borrow_mut().remove_window(&event.window);
    }

    //Note to get general atoms look at
    //https://github.com/sminez/penrose/blob/develop/src/x11rb/mod.rs lines 404-500
    pub fn atom_window_type_dock(&self, winid: u32) -> bool {
        let atom_id = get_internal_atom(&self.connection, Atom::NetWmWindowType.as_ref());

        self.connection.flush().unwrap();
        let atom_reply = self
            .connection
            .get_property(false, winid, atom_id, AtomEnum::ANY, 0, 1024)
            .unwrap()
            .reply();
        if let Ok(atom_reply) = atom_reply {
            self.connection.flush().unwrap();

            let prop_type = match atom_reply.type_ {
                0 => return false, // Null response
                atomid => atom_name(&self.connection, atomid),
            };

            let wm_type = Atom::NetWindowTypeDock.as_ref();
            if prop_type == "ATOM" {
                let atoms = atom_reply
                    .value32()
                    .unwrap()
                    .map(|a| atom_name(&self.connection, a))
                    .collect::<Vec<String>>();
                if atoms.contains(&wm_type.to_string()) {
                    info!("spawned window is of type _NET_WM_WINDOW_TYPE_DOCK");
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
