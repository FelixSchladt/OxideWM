pub mod commands;
pub mod events;

use self::events::{IpcEvent, WmActionEvent};

use log::info;
use x11rb::protocol::{Event, xproto::{KeyPressEvent, ModMask}};
use std::process;

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
        let log_msg = "Received Event: ";
        match event {
            Event::Expose(_event) => info!("{} Expose", log_msg),
            Event::UnmapNotify(_event) => {
                info!("{} UnmapNotify", log_msg);
                self.window_manager.handle_event_unmap_notify(_event);
            },
            Event::ButtonPress(_event) => info!("{} ButtonPress", log_msg),
            Event::MotionNotify(_event) => info!("{} MotionNotify", log_msg),
            Event::ButtonRelease(_event) => info!("{} ButtonRelease", log_msg),
            Event::ConfigureRequest(_event) => info!("{} ConfigureRequest", log_msg),
            Event::MapRequest(_event) => {
                info!("{} MapRequest", log_msg);
                self.window_manager.handle_map_request(_event);
            },
            Event::KeyPress(_event) => info!("{} KeyPress", log_msg),
            Event::KeyRelease(_event) => {
                info!("{} KeyPress", log_msg);
                self.handle_keypress(_event);
            },
            Event::DestroyNotify(_event) => info!("{} DestroyNotify", log_msg),
            Event::PropertyNotify(_event) => info!("{} PropertyNotify", log_msg),
            Event::EnterNotify(_event) => {
                info!("{} EnterNotify!!!", log_msg);
                self.window_manager.handle_event_enter_notify(_event);
            },
            Event::LeaveNotify(_event) => {
                info!("{} LeaveNotify", log_msg);
                self.window_manager.handle_event_leave_notify(_event);
            },
            Event::FocusIn(_event) => info!("{} FocusIn", log_msg),
            Event::FocusOut(_event) => info!("{} FocusOut", log_msg),
            _ => info!("{} \x1b[33mUnknown\x1b[0m {:?}", log_msg, event),
        };
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) {
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
            if state == key.keycode.mask || state == key.keycode.mask | u16::from(ModMask::M2) {
                println!("Key: {:?}", key);
                self.handle_wm_command(WmActionEvent {
                    command: key.event,
                    args: key.args.clone(),
                });
            }
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
