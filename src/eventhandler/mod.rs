pub mod commands;
use crate::{
    windowmanager::{WindowManager, WmActionEvent, IpcEvent},
    keybindings::KeyBindings, 
    auxiliary::exec_user_command, 
    eventhandler::commands::WmCommands,
};

use log::info;
use x11rb::protocol::{Event, xproto::{KeyPressEvent, ModMask}};

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
            _ => println!("\x1b[33mUnknown\x1b[0m {:?}", event),
        };
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) {
        let keys = self.keybindings.events_map
            .get(&event.detail);
        if keys.is_none() {
            // the keys are empty, when a keybinding [alt, t] for example exists.
            // if the user then holds t and only then presses alt, the event.details, are for the alt key
            // since we search for the bindings by the details, and check the mask later, the binding is not found. 
            info!("No keybinding found grabbed keybinding, will not be handled");
            return;
        }

        for key in keys.unwrap().clone() {
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
            self.handle_wm_command(command);
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
