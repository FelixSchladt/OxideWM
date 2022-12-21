use std::process::exit;
use std::error::Error;
use std::collections::HashMap;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::Event;
use x11rb::connection::Connection;
use x11rb::protocol::ErrorKind;
use x11rb::errors::{
    ReplyError,
    ConnectionError,
};
use x11rb::protocol::xproto::{
    ConnectionExt,
    Screen,
    ChangeWindowAttributesAux,
    EventMask,
};
use crate::workspace::Workspace;


#[derive(Debug)]
pub struct ScreenInfo {
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub width: u16,
    pub height: u16,
}

impl ScreenInfo {
    pub fn window_exists(&self, winid: u32) -> bool {
        for workspace in &self.workspaces {
            if workspace.windows.contains_key(&winid) {
                return true;
            }
        }
        false
    }

    pub fn map_request(&mut self, connection: &RustConnection, event: &MapRequestEvent) {
        println!("WINMAN: MapRequestEvent: {:?}", event);
        let mut workspace = &mut self.workspaces[self.active_workspace.clone()];
        //TODO: Check if window is already in workspace
        /*
        if self.window_exists(event.window) {
            workspace.new_window(connection, event.window);
        }*/
        workspace.new_window(connection, event.window);
        workspace.remap_windows(connection);
    }
}

#[derive(Debug)]
pub struct WindowManager {
    pub connection: RustConnection,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    //config: Config,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let (connection, screen_index) = RustConnection::connect(None).unwrap();
        let screeninfo = HashMap::new();
        let mut manager = WindowManager {
            connection,
            screeninfo,
        };

        manager.setup_screens();

        manager.update_root_window_event_masks();

        manager
    }
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
                self.screeninfo.get_mut(&_event.parent).unwrap().map_request(&self.connection, _event);
            },
            _ => println!("\x1b[33mUnknown\x1b[0m"),
        };
}

    fn setup_screens(&mut self) {
        //TODO remove unneccessar multicall on this function
        for screen in self.connection.setup().roots.iter() {
            let mut screenstruct = ScreenInfo {
                workspaces: Vec::new(),
                active_workspace: 0,
                width: screen.width_in_pixels,
                height: screen.height_in_pixels,
            };
            screenstruct.workspaces.push(Workspace::new(0));
            self.screeninfo.insert(screen.root, screenstruct);
        }
    }

    fn update_root_window_event_masks(&self) {
        let mask = ChangeWindowAttributesAux::default()
                   .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY
                    );

        for screen in self.connection
                          .setup()
                          .roots
                          .iter() {
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
        let update_result = self.connection.change_window_attributes(
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
}
