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
use std::process::{Command, Stdio};

use crate::screeninfo::ScreenInfo;
use crate::workspace::Workspace;
use crate::config::{Config, WmCommands};
use crate::keybindings::KeyBindings;

#[derive(Debug)]
pub struct WindowManager {
    pub connection: Rc<RefCell<RustConnection>>,
    pub screeninfo: HashMap<u32, ScreenInfo>,
    pub config: Rc<RefCell<Config>>,
    pub keybindings: KeyBindings,
    pub focused_screen: u32,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let connection = Rc::new(RefCell::new(RustConnection::connect(None).unwrap().0));
        let screeninfo = HashMap::new();
        let config = Rc::new(RefCell::new(Config::new()));
        let keybindings = KeyBindings::new(&config.borrow());

        let focused_screen = 0; 
        //TODO: Get focused screen from X11
        // Currently the screen setup last is taken as active.
        // We should discuss if this default behaviour is ok or not.

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

        manager.connection.borrow_mut().flush().unwrap();

        manager
    }
    
    fn handle_keypress_kill(&mut self) {
        let active_workspace = self.screeninfo
            .get(&self.focused_screen)
            .unwrap().active_workspace;
        let current_window = self.screeninfo
            .get(&self.focused_screen)
            .unwrap().workspaces[active_workspace]
            .get_focused_window();
        println!("Current window: {:?}", current_window);
        if let Some(winid) = current_window {
            self.screeninfo
                .get_mut(&self.focused_screen)
                .unwrap().workspaces[active_workspace]
                .kill_window(&winid);
        } else {
            println!("ERROR: No window to kill \nShould only happen on an empty screen");
        }
    }


    fn handle_keypress(&mut self, event: &KeyPressEvent) {
        //TODO make sure a spawned window is spawned on the correct screen/workspace?
        let keys = self.keybindings.events_map
            .get(&event.detail)
            .expect("ERROR: Key not found in keybindings -> THIS MUST NOT HAPPEN");
        //NOTE: IF you get the error above, this is probably cause by an inconsistency
        // in the Connection. Most likely you did something with the connection that
        // left it in a weird state. This **must not be** directly connected to this
        // function. Maybe a flush helps but check if there is something else wrong
        // with your changes. I experienced this a couple of times and it always was
        // quite strange and hard to find. Ask for help if you can't find the problem.

        for key in keys.clone() {
            let state = u16::from(event.state);
            if state == key.keycode.mask 
            || state == key.keycode.mask | u16::from(ModMask::M2) {
                println!("Key: {:?}", key);
                match key.event {
                    WmCommands::Move => {
                        println!("Move");
                    },
                    WmCommands::Resize => {
                        println!("Resize");
                    },
                    WmCommands::Quit => {
                        println!("Quit");
                    },
                    WmCommands::Kill => {
                        println!("Kill");
                        self.handle_keypress_kill();
                    },
                    WmCommands::Restart => {
                        println!("Restart");
                    },
                    WmCommands::Exec => {
                        println!("Exec");
                        exec_user_command(&key.args);
                    },
                    _ => {
                        println!("Unimplemented");
                    }
                }
            }
        }
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
            self.focused_screen = screen.root;
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

    fn handle_event_enter_notify(&mut self, event: &EnterNotifyEvent) {
        self.focused_screen = event.root;
        let workspace_id = self.screeninfo
            .get(&event.root)
            .unwrap()
            .active_workspace;
        self.screeninfo
            .get_mut(&event.root)
            .unwrap()
            .workspaces[workspace_id]
            .focus_window(event.event);
    }

    fn handle_event_leave_notify(&mut self, event: &LeaveNotifyEvent) {
        let workspace_id = self.screeninfo
            .get(&event.root)
            .unwrap().active_workspace;
        self.screeninfo
            .get_mut(&event.root)
            .unwrap().workspaces[workspace_id]
            .unfocus_window(event.event);
    }


    fn handle_event_unmap_notify(&mut self, event: &UnmapNotifyEvent) {
        let workspace_id = self.screeninfo
            .get(&event.event)
            .unwrap().active_workspace;
        self.screeninfo
            .get_mut(&event.event)
            .unwrap().workspaces[workspace_id]
            .remove_window(&event.window);
    }

    pub fn handle_event(&mut self, event: &Event) {
        //TODO: move the events into seperate functions
        print!("Received Event: ");
        match event {
            Event::Expose(_event) => println!("Expose"),
            Event::UnmapNotify(_event) => {
                println!("UnmapNotify");
                self.handle_event_unmap_notify(_event);
           },
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
            Event::PropertyNotify(_event) => println!("PropertyNotify"),
            Event::EnterNotify(_event) => {
                //println!("EnterNotify!!!");
                self.handle_event_enter_notify(_event);
           },
            Event::LeaveNotify(_event) => {
                //println!("LeaveNotify");
                self.handle_event_leave_notify(_event);
            },
            Event::FocusIn(_event) => println!("FocusIn"),
            Event::FocusOut(_event) => println!("FocusOut"),
            _ => println!("\x1b[33mUnknown\x1b[0m {:?}", event),
        };
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

///TODO Maybe move this to a separate file
pub fn exec_user_command(args: &Option<String>) {
    match args {
        Some(args) => {
            let mut args = args.split_whitespace();
            let command = args.next().unwrap();
            let args = args.collect::<Vec<&str>>().join(" ");
            if args.is_empty() {
                Command::new(command)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            } else {
                Command::new(command)
                    .arg(args)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            }.unwrap();
        },
        None => panic!("User command called without args"),
    }
}
