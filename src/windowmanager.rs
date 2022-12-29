use std::process::exit;
<<<<<<< HEAD
use std::error::Error;

use super::workspace::Workspace;
use super::windowstate::WindowState;

use x11rb::COPY_DEPTH_FROM_PARENT;
=======
use std::collections::HashMap;
use x11rb::protocol::xproto::*;
>>>>>>> feature/ISSUE29-skeleton
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::Event;
use x11rb::connection::Connection;
use x11rb::protocol::ErrorKind;
use x11rb::protocol::xproto::{
    ConnectionExt,
    Screen,
    Window,
    ChangeWindowAttributesAux,
    ConfigureWindowAux,
    CreateWindowAux,
    EventMask,
};
use crate::workspace::Workspace;
use x11rb::rust_connection::ReplyError;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ScreenInfo {
    pub connection: Rc<RefCell<RustConnection>>,
    pub id: u32,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u16,
    pub height: u16,
}

impl ScreenInfo {
    pub fn new(connection: Rc<RefCell<RustConnection>>, id: u32, height: u16, width: u16) -> ScreenInfo {
        let active_workspace = 0;
        let workspaces = Vec::new();
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
}

#[derive(Debug)]
pub struct WindowManager {
<<<<<<< HEAD
    pub connection: RustConnection,
    active_monitor_id: u16,
    //pub monitors: HashMap<u16, Vec<Workspace>>,
=======
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
>>>>>>> feature/ISSUE29-skeleton
    //config: Config,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
        let screeninfo = HashMap::new();
        let mut manager = WindowManager {
            connection,
            screeninfo,
        };

        manager.setup_screens();
        manager.update_root_window_event_masks();

        manager
    }

<<<<<<< HEAD
    pub fn map_window(window: Window, monitor_id: u16, workspace_id: u16) {
         
    }

    pub fn remap(monitor_id: u16, workspace_id: u16) {
        //TODO implement method parameters
=======
    pub fn handle_event(&mut self, event: &Event) {
        print!("Received Event: ");
        match event {
            Event::Expose(_event) => println!("Expose"),
            Event::UnmapNotify(_event) => println!("UnmapNotify"),
            Event::EnterNotify(_event) => println!("EnterNotify"),
            Event::ButtonPress(_event) => println!("ButtonPress"),
            Event::MotionNotify(_event) => println!("MotionNotify"),
            Event::ButtonRelease(_event) => println!("ButtonRelease"),
            Event::ConfigureRequest(_event) => println!("ConfigureRequest"),
            Event::MapRequest(_event) => {
                println!("MapRequest");
                self.screeninfo.get_mut(&_event.parent).unwrap().on_map_request(_event);
            },
            _ => println!("\x1b[33mUnknown\x1b[0m"),
        };
    }

    fn setup_screens(&mut self) {
        //TODO remove unneccessar multicall on this function
        for screen in self.connection.borrow().setup().roots.iter() {
            let mut screenstruct = ScreenInfo::new(self.connection.clone(),
                                                   screen.root,
                                                   screen.width_in_pixels,
                                                   screen.height_in_pixels,
                                                   );
            screenstruct.workspaces.push(Workspace::new(0,
                                                        self.connection.clone(),
                                                        0,
                                                        0,
                                                        screen.width_in_pixels as u32,
                                                        screen.height_in_pixels as u32,
                                                        ));
            self.screeninfo.insert(screen.root, screenstruct);
        }
>>>>>>> feature/ISSUE29-skeleton
    }

    fn update_root_window_event_masks(&self) {
        let mask = ChangeWindowAttributesAux::default()
                   .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY
                    );

        for screen in self.connection.borrow().setup().roots.iter() {
            #[cfg(debug_assertion)]
            println!("Attempting to update event mask of: {} -> ", screen.root);
            self.set_mask(screen, mask).unwrap();
            println!("Screen: {} -> {}", screen.root, screen.width_in_pixels);
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

        #[cfg(debug_assertion)]
        match update_result {
             Ok(_) => println!("\x1b[32mSuccess\x1b[0m"),
             Err(_) => println!("\x1b[31mFailed\x1b[0m"),
        }

        update_result
    }

    fn create_windowstate(&self) -> Result<WindowState, Box<dyn Error>> {
        let frame_id = self.connection.generate_id()?;
        let titlebar_id = self.connection.generate_id()?;

        let window_aux = ConfigureWindowAux::default()
                         .width(100)
                         .height(100)
                         .x(10)
                         .y(10);

        self.connection.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_id,
            self.,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().background_pixel(screen.white_pixel),
        )?;

        Ok(WindowState {
            title: format!("{}", frame_id),
            visible: true,
            focused: true,
            urgent: false,
            titlebar_height: 0,
        })
    }
}
