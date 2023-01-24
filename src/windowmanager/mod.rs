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
    atom::Atom,
    workspace::{
        Workspace,
        enums_workspace::{Layout,GoToWorkspace},
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
        info!("grabbing keys");
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
                    workspace.move_focus(movement);
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
                    workspace.move_window(movement);
                },
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
        let screen_option = self.screeninfo
            .get_mut(&self.focused_screen);
        if screen_option.is_none() {
            warn!("Could not switch workspace, no screen was focused");
            return;
        }

        let arg;
        if let Some(args) = args_option {
            let go_to_result = GoToWorkspace::try_from(args.as_str());
            match go_to_result {
                Ok(go_to) => arg=go_to,
                Err(_) => {
                    warn!("Argumet '{}' could not be parsed", args);
                    return;
                },
            }
        }else{
            warn!("No argument was passed");
            return;
        }

        let screen= screen_option.unwrap();
        
        let max_workspace = screen.get_workspace_count() - 1;
        let active_workspace = screen.active_workspace;
        let new_workspace = arg.calculate_new_workspace(active_workspace as usize, max_workspace);
        screen.set_workspace_create_if_not_exists(new_workspace as u16);
    }

    pub fn handle_keypress_fullscreen(&mut self) {
        self.get_active_workspace().toggle_fullscreen();
    }

    fn setup_screens(&mut self) {
        for screen in self.connection.borrow().setup().roots.iter() {
            let screen_ref = Rc::new(RefCell::new(screen.clone()));
            let mut screenstruct = ScreenInfo::new(
                self.connection.clone(),
                screen_ref.clone(),
                screen.width_in_pixels as u32,
                screen.height_in_pixels as u32,
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

        for screen in self.connection.borrow().setup().roots.iter() {
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
                error!("Access to X11 Client Api denied!");
                exit(1);
            }
        }

        update_result
    }

    pub fn handle_event_enter_notify(&mut self, event: &EnterNotifyEvent) {
        let mut winid = event.event;
        if self.moved_window.is_some() {
            winid =  self.moved_window.unwrap();
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
        let reply = self.connection.borrow().get_atom_name(id).unwrap().reply().unwrap();
        self.connection.borrow().flush().unwrap();
        String::from_utf8(reply.name).unwrap()
    }

    //Note to get general atoms look at
    //https://github.com/sminez/penrose/blob/develop/src/x11rb/mod.rs lines 404-500
    pub fn atom_window_type_dock(&self, winid: u32) -> bool {
        let binding = self.connection.borrow();
        let atom_intern = binding.intern_atom(false, Atom::NetWmWindowType.as_ref().as_bytes()).unwrap().reply().unwrap().atom;

        self.connection.borrow().flush().unwrap();
        let atom_reply = binding.get_property(false,
                                              winid,
                                              atom_intern,
                                              AtomEnum::ANY,
                                              0,
                                              1024).unwrap().reply();
        if let Ok(atom_reply) = atom_reply {
            self.connection.borrow().flush().unwrap();

            let prop_type = match atom_reply.type_ {
                0 => return false, // Null response
                atomid => self.atom_name(atomid),
            };

            let wm_type = Atom::NetWindowTypeDock.as_ref();
            if prop_type == "ATOM" {
                let atoms = atom_reply.value32().unwrap()
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
            self.screeninfo.get_mut(&event.parent.clone()).unwrap().add_status_bar(event);
        }
    }

    pub fn handle_map_request(&mut self, event: &MapRequestEvent) {
        if !self.atom_window_type_dock(event.window.clone()) {
            self.screeninfo.get_mut(&event.parent.clone()).unwrap().on_map_request(event);
        } 
    }
}
