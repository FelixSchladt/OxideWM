pub mod enums_windowmanager;

use self::enums_windowmanager::Movement;

use std::collections::HashMap;
use std::error::Error;
use std::process::exit;
use std::{cell::RefCell, rc::Rc};
use serde::Serialize;

use log::{warn, error, info, debug};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{
    protocol::{
        ErrorKind,
        Event
    },
    protocol::xproto::*,
    rust_connection::{
        ConnectionError,
        RustConnection,
        ReplyError
    }
};

use crate::{
    auxiliary::exec_user_command,
    keybindings::KeyBindings,
    screeninfo::ScreenInfo,
    config::Config,
    workspace::{
        Workspace,
        enums::{Layout,GoToWorkspace},
    }
};

#[derive(Debug, Serialize)]
pub struct WindowManagerState {
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Config,
    pub focused_screen: u32,
}

#[derive(Debug, Clone)]
pub struct WindowManager {
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub focused_screen: u32,
    pub moved_window: Option<u32>,
    pub restart: bool,
}


impl WindowManager {
    pub fn new(keybindings: &KeyBindings, config: Rc<RefCell<Config>>) -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
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
        manager.update_root_window_event_masks();
        manager.grab_keys(keybindings).expect("Failed to grab Keys");
        let result = manager.connection.borrow().flush();
        if result.is_err() {
            info!("Failed to flush rust connection");
        }

        manager.autostart_exec();
        manager.autostart_exec_always();
        manager.connection.borrow_mut().flush().unwrap();

        manager
    }

    pub fn restart_wm(&mut self, keybindings: &KeyBindings, config: Rc<RefCell<Config>>) {
        self.config = config;
        //self.keybindings = KeyBindings::new(&self.config.borrow());
        self.grab_keys(keybindings).expect("Failed to grab Keys");
        self.autostart_exec_always();
        self.connection.borrow_mut().flush().unwrap();
        self.restart = false;
    }
    fn autostart_exec(&self) {
        for command in &self.config.borrow().exec {
            exec_user_command(&Some(command.clone()));
        }
    }

    fn autostart_exec_always(&self) {
        for command in &self.config.borrow().exec_always {
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

    fn grab_keys(&self, keybindings: &KeyBindings) -> Result<(), Box<dyn Error>> {
        println!("grabbing keys");
        //TODO check if the the screen iterations should be merged
        for screen in self.connection.borrow().setup().roots.iter() {
            for modifier in [0, u16::from(ModMask::M2)] {
                for keyevent in keybindings.events_vec.iter() {
                    self.connection.borrow().grab_key(
                        false,
                        screen.root,
                        (keyevent.keycode.mask | modifier).into(),
                        keyevent.keycode.code,
                        GrabMode::ASYNC,
                        GrabMode::ASYNC,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn get_active_workspace_id(&self) -> u16 {
        return self.screeninfo.get(&self.focused_screen).unwrap().active_workspace;
    }

    fn get_active_workspace(&mut self) -> &mut Workspace {
        let active_workspace_id = self.get_active_workspace_id();
        let screen_info = self.screeninfo.get_mut(&self.focused_screen).unwrap();
        screen_info.get_workspace(active_workspace_id)
    }

    fn get_focused_window(&mut self) -> Option<u32> {
        let workspace = self.get_active_workspace();
        workspace.get_focused_window()
    }

    pub fn poll_for_event(&self)->Result<Option<Event>, ConnectionError>{
        self.connection.borrow().poll_for_event()
    }

    pub fn handle_keypress_focus(&mut self, args_option: Option<String>) {
        if let Some(args) = args_option {
            match Movement::try_from(args.as_str()) {
                Ok(movement) => {
                    let workspace = self.get_active_workspace();
                    workspace.move_focus(&movement);
                },
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
                    workspace.move_window(&movement);
                },
                Err(_) => warn!("Could not parse movement from argument {}", args),
            }
        } else {
            warn!("Argument must be provided");
        }
    }

    pub fn handle_keypress_kill(&mut self) {
        let focused_window = self.get_focused_window();
        println!("Focused window: {:?}", focused_window);
        if let Some(winid) = focused_window {
            self.get_active_workspace()
                .kill_window(&winid);
        } else {
            error!("ERROR: No window to kill \nShould only happen on an empty screen");
        }
    }

    pub fn handle_keypress_layout(&mut self, args: Option<String>) {
        let active_workspace = self.get_active_workspace();

        match args {
            Some(args) => {
                let layout = Layout::try_from(args.as_str());
                if layout.is_err(){
                    warn!("Layout could not be parsed from argument {}", args);
                    return;
                }
                active_workspace.set_layout(layout.unwrap());
            },
            None => warn!("No argument provided"),
        }
    }

    pub fn handle_keypress_go_to_workspace(&mut self, args_option: Option<String>){
        let screen = if let Some(screen) = self.screeninfo.get_mut(&self.focused_screen) { screen } else {
            error!("Failed to get screen.");
            return;
        };

        let arg = if let Some(args) = args_option {
            if let Ok(direction) = GoToWorkspace::try_from(args.as_str()) { direction } else {
                 warn!("Argumet '{}' could not be parsed", args);
                 return;
            }
        }else{
            warn!("No argument was passed");
            return;
        };

        let max_workspace = screen.get_workspace_count() - 1;
        let active_workspace = screen.active_workspace;
        let new_workspace = arg.calculate_new_workspace(active_workspace.into(), max_workspace);
        screen.set_workspace_create_if_not_exists(u16::try_from(new_workspace).unwrap());
    }

    fn setup_screens(&mut self) {
        for screen in &self.connection.borrow().setup().roots {
            let screen_ref = Rc::new(RefCell::new(screen.clone()));
            let mut screenstruct = ScreenInfo::new(
                self.connection.clone(),
                screen_ref.clone(),
                u32::from(screen.width_in_pixels),
                u32::from(screen.height_in_pixels),
            );
            screenstruct.create_new_workspace();    // Todo Js remove this
            self.screeninfo.insert(screen.root, screenstruct);
            self.focused_screen = screen.root;
            debug!("screen widht: {} screen height: {}", screen.width_in_pixels, screen.height_in_pixels);
        }
    }

    fn update_root_window_event_masks(&self) {
        let mask = ChangeWindowAttributesAux::default()
                   .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY |
                        EventMask::BUTTON_MOTION |
                        EventMask::FOCUS_CHANGE |
                        //EventMask::ENTER_WINDOW |
                        //EventMask::LEAVE_WINDOW | //this applies only to the rootwin
                        EventMask::PROPERTY_CHANGE
                    );

        for screen in &self.connection.borrow().setup().roots {
            self.set_mask(screen, mask).unwrap();
        }
    }

    fn set_mask(
        &self,
        screen: &Screen,
        mask: ChangeWindowAttributesAux
    ) -> Result<(), ReplyError> {
        let update_result = self.connection.borrow().change_window_attributes(
                                screen.root,
                                &mask
                            )?.check();

        if let Err(ReplyError::X11Error(ref error)) = update_result {
            if error.error_kind == ErrorKind::Access {
                eprintln!("\x1b[31m\x1b[1mError:\x1b[0m Access to X11 Client Api denied!");
                exit(1);
            }
        }
        update_result
    }

    pub fn handle_event_enter_notify(&mut self, enter_notify: &EnterNotifyEvent) {
        let winid = if let Some(window) = self.moved_window {
            self.moved_window = None;
            window
        } else {
            enter_notify.event
        };

        self.get_active_workspace().focus_window(winid);
    }

    pub fn handle_event_leave_notify(&mut self, _event: &LeaveNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.unfocus_window();
    }


    pub fn handle_event_unmap_notify(&mut self, event: &UnmapNotifyEvent) {
        let active_workspace = self.get_active_workspace();
        active_workspace.remove_window(&event.window);
    }

    pub fn handle_map_request(&mut self, event: &MapRequestEvent) {
        let screeninfo = if let Some(screeninfo) = self.screeninfo.get_mut(&event.parent) { screeninfo } else {
            error!("Failed to get screeninfo.");
            exit(-1);
        };

        screeninfo.on_map_request(event);
    }
}
