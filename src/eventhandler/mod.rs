pub mod commands;
pub mod events;

use self::events::{IpcEvent, WmActionEvent};

use x11rb::protocol::{Event, xproto::{KeyPressEvent, ModMask}};
use std::process;
use log::error;

use crate::{
    windowmanager::{WindowManager},
    keybindings::KeyBindings, 
    auxiliary::exec_user_command, 
    eventhandler::commands::WmCommands,
};


pub struct EventHandler<'a>{
    pub window_manager: &'a mut WindowManager,
    keybindings: &'a KeyBindings,
}

impl EventHandler<'_> {
    pub fn new<'a>(window_manager: &'a mut WindowManager, keybindings: &'a KeyBindings)->EventHandler<'a>{
        EventHandler{
            window_manager,
            keybindings
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        print!("Received Event: ");
        match event {
            Event::Expose(_event) => println!("Expose"),
            Event::UnmapNotify(_event) => {
                println!("UnmapNotify");
                self.window_manager.handle_event_unmap_notify(_event);
            },
            Event::ButtonPress(_event) => println!("ButtonPress"),
            Event::MotionNotify(_event) => println!("MotionNotify"),
            Event::ButtonRelease(_event) => println!("ButtonRelease"),
            Event::ConfigureRequest(_event) => println!("ConfigureRequest"),
            Event::MapRequest(_event) => {
                println!("MapRequest");
                self.window_manager.handle_map_request(_event);
            },
            Event::KeyPress(_event) => println!("KeyPress"),
            Event::KeyRelease(_event) => {
                println!("KeyPress");
                self.handle_keypress(_event);
            },
            Event::DestroyNotify(_event) => println!("DestroyNotify"),
            Event::PropertyNotify(_event) => println!("PropertyNotify"),
            Event::EnterNotify(_event) => {
                println!("EnterNotify!!!");
                self.window_manager.handle_event_enter_notify(_event);
            },
            Event::LeaveNotify(_event) => {
                println!("LeaveNotify");
                self.window_manager.handle_event_leave_notify(_event);
            },
            Event::FocusIn(_event) => println!("FocusIn"),
            Event::FocusOut(_event) => println!("FocusOut"),
            Event::CreateNotify(_event) => {
                println!("CreateNotify");
                self.window_manager.handle_create_notify(_event);
            },
            _ => println!("\x1b[33mUnknown\x1b[0m {:?}", event),
        };
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) {
        match self.keybindings.events_map.get(&event.detail) {
            Some(keys) => {
        //NOTE: IF you get the error above, this is probably cause by an inconsistency
        // in the Connection. Most likely you did something with the connection that
        // left it in a weird state. This **must not be** directly connected to this
        // function. Maybe a flush helps but check if there is something else wrong
        // with your changes. I experienced this a couple of times and it always was
        // quite strange and hard to find. Ask for help if you can't find the problem.

                for key in keys.clone() {
                    let state = u16::from(event.state);
                    if state == key.keycode.mask || state == key.keycode.mask | u16::from(ModMask::M2) {
                        println!("Key: {:?}", key);
                        self.handle_wm_command(WmActionEvent {
                            command: key.event,
                            args: key.args.clone(),
                        });
                    }
                }
            },
            None => error!("Key not found: {:?} if this happens frequently, you probably left the X connection in a weird state", event.detail),
        }
    }

    pub fn handle_ipc_event(&mut self, event: IpcEvent) {
        println!("IpcEvent: {:?}", event);
        if let Some(command) = event.event {
            self.handle_wm_command(command)
        }
    }

    fn handle_wm_command(&mut self, command: WmActionEvent) {
         match command.command {
            WmCommands::Move => {
                println!("Move");
                self.window_manager.handle_keypress_move(command.args.clone());
            },
            WmCommands::Focus => {
                println!("Focus");
                self.window_manager.handle_keypress_focus(command.args.clone());
            },
            WmCommands::Resize => {
                println!("Resize");
            },
            WmCommands::Quit => {
                 println!("Quit");
                 process::exit(0);
            },
            WmCommands::Kill => {
                println!("Kill");
                self.window_manager.handle_keypress_kill();
            },
            WmCommands::Layout => {
                println!("Layout");
                self.window_manager.handle_keypress_layout(command.args.clone());
            },
            WmCommands::Restart => {
                println!("Restart");
                self.window_manager.restart = true;
            },
            WmCommands::GoToWorkspace =>{
                self.window_manager.handle_keypress_go_to_workspace(command.args.clone());
            },
            WmCommands::Exec => {
                println!("Exec");
                exec_user_command(&command.args);
            },
            _ => {
                println!("Unimplemented");
            }
        }
    }
}
