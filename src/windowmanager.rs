use std::process::exit;
use std::collections::HashMap;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::{
    Event,
    ErrorKind
};
use x11rb::rust_connection::{
    RustConnection,
    ReplyError
};

use crate::screeninfo::ScreenInfo;
use crate::workspace::Workspace;
use crate::config::Config;
use crate::keybindings::KeyBindings;

#[derive(Debug)]
pub struct WindowManager {
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub keybindings: KeyBindings,
    pub focused_screen: u32,
    //config: Config,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
        let screeninfo = HashMap::new();
        let config = Rc::new(RefCell::new(Config::new()));
        let keybindings = KeyBindings::new(&config.borrow());

        let focused_screen = 0; //TODO: Get focused screen from X11

        let mut manager = WindowManager {
            connection,
            screeninfo,
            config,
            keybindings,
            focused_screen,
        };

        manager.setup_screens();
        manager.update_root_window_event_masks();
        manager.grab_keys().unwrap();

        get_cursor_position(&manager);

        manager
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) {
        //how do we make sure a spawned window is spawned on the correct screen/workspace?
        let keys = self.keybindings.events_map.get(&event.detail).expect("Registered key not found");
        for key in keys {
            let state = u16::from(event.state);
            if state == key.keycode.mask 
            || state == key.keycode.mask | u16::from(ModMask::M2) {
                println!("Key: {:?}", key);
                (key.event)(key.args.clone());
            }
        };
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
                self.screeninfo.get_mut(&_event.parent).unwrap().on_map_request(_event);
            },
            Event::KeyPress(_event) => println!("KeyPress"),
            Event::KeyRelease(_event) => {
                println!("KeyPress");
                self.handle_keypress(_event);
            },
            Event::DestroyNotify(_event) => println!("DestroyNotify"),
            Event::EnterNotify(_event) => println!("EnterNotify"),
            Event::MotionNotify(_event) => println!("MotionNotify"),
            _ => println!("\x1b[33mUnknown\x1b[0m"),
        };
    }

    fn setup_screens(&mut self) {
        //TODO remove unneccessar multicall on this function
        //check if the the screen iterations should be merged
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
    }

    fn update_root_window_event_masks(&self) {
        let mask = ChangeWindowAttributesAux::default()
                   .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY
                    );

        for screen in self.connection.borrow().setup().roots.iter() {
            //TODO check if the the screen iterations should be merged
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

    fn grab_keys(&self) -> Result<(), Box<dyn Error>> {
        for screen in self.connection.borrow().setup().roots.iter() {
            //TODO check if the the screen iterations should be merged
            for modifier in [0, u16::from(ModMask::M2)] {
                for keyevent in self.keybindings.events_vec.iter() {
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
}

pub struct Coordinates {
    x: i16,
    y: i16,
}

pub fn get_cursor_position(winman: &WindowManager) -> Result<Coordinates, Box<dyn Error>> {
    for screen in winman.connection.borrow().setup().roots.iter() {
        let reply = winman.connection.borrow().query_pointer(screen.root)?.reply()?;
        println!("Reply: {:?}", reply);
    }
    //println!("Cursor: {} {}", reply.root_x, reply.root_y);
    Ok(Coordinates { x: 0, y: 0 })
}
