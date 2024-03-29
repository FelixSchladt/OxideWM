pub mod events;

use self::events::{EventType, IpcEvent};

use log::error;
use log::{debug, info, trace};
use oxide_common::ipc::action_event::WmActionEvent;
use oxide_common::ipc::commands::WmCommands;
use std::{
    process,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};
use x11rb::protocol::{
    xproto::{KeyPressEvent, ModMask},
    Event,
};

use crate::{auxiliary::exec_user_command, keybindings::KeyBindings, windowmanager::WindowManager};

pub struct EventHandler<'a> {
    pub window_manager: &'a mut WindowManager,
    keybindings: &'a KeyBindings,
}

impl EventHandler<'_> {
    pub fn new<'a>(
        window_manager: &'a mut WindowManager,
        keybindings: &'a KeyBindings,
    ) -> EventHandler<'a> {
        EventHandler {
            window_manager,
            keybindings,
        }
    }

    pub fn run_event_loop(
        &mut self,
        receive_channel: Arc<Mutex<Receiver<EventType>>>,
        status_send_channel: Arc<Mutex<Sender<String>>>,
    ) {
        loop {
            if let Ok(event_type) = receive_channel.lock().unwrap().recv() {
                match event_type {
                    EventType::X11rbEvent(event) => self.handle_x_event(&event),
                    EventType::OxideEvent(event) => {
                        self.handle_ipc_event(event, status_send_channel.clone())
                    }
                }
                debug!("ready to receive another event");
            }

            if self.window_manager.restart {
                info!("exeting event loop");
                break;
            }
        }
    }

    fn handle_x_event(&mut self, event: &Event) {
        let log_msg = "Received Event: ";
        match event {
            Event::Expose(_event) => info!("{} Expose", log_msg),
            Event::UnmapNotify(_event) => info!("{} UnmapNotify", log_msg),
            Event::ButtonPress(_event) => info!("{} ButtonPress", log_msg),
            Event::MotionNotify(_event) => info!("{} MotionNotify", log_msg),
            Event::ButtonRelease(_event) => info!("{} ButtonRelease", log_msg),
            Event::ConfigureRequest(_event) => info!("{} ConfigureRequest", log_msg),
            Event::MapRequest(_event) => {
                info!("{} MapRequest", log_msg);
                self.window_manager.handle_map_request(_event);
            }
            Event::KeyPress(_event) => info!("{} KeyPress", log_msg),
            Event::KeyRelease(_event) => {
                info!("{} KeyPress", log_msg);
                self.handle_keypress(_event);
            }
            Event::DestroyNotify(_event) => {
                info!("{} DestroyNotify", log_msg);
                self.window_manager.handle_event_destroy_notify(_event);
            }
            Event::PropertyNotify(_event) => info!("{} PropertyNotify", log_msg),
            Event::EnterNotify(_event) => {
                info!("{} EnterNotify!!!", log_msg);
                self.window_manager.handle_event_enter_notify(_event);
            }
            Event::LeaveNotify(_event) => {
                info!("{} LeaveNotify", log_msg);
                self.window_manager.handle_event_leave_notify(_event);
            }
            Event::FocusIn(_event) => info!("FocusIn"),
            Event::FocusOut(_event) => info!("FocusOut"),
            Event::CreateNotify(_event) => {
                println!("CreateNotify");
                self.window_manager.handle_create_notify(_event);
            }
            _ => info!("{} Unknown {:?}", log_msg, event),
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
                        debug!("Key: {:?}", key);
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

    fn handle_ipc_event(
        &mut self,
        event: IpcEvent,
        status_send_channel: Arc<Mutex<Sender<String>>>,
    ) {
        trace!("IpcEvent: {:?}", event);
        if let Some(command) = event.event {
            self.handle_wm_command(command)
        }

        if event.status {
            let wm_state = self.window_manager.get_state();
            let j = serde_json::to_string(&wm_state).unwrap();
            trace!("IPC status request");
            status_send_channel.lock().unwrap().send(j).unwrap();
        }
    }

    fn handle_wm_command(&mut self, command: WmActionEvent) {
        info!("Handle wm command {command}");
        match command.command {
            WmCommands::Move => self
                .window_manager
                .handle_keypress_move(command.args.clone()),
            WmCommands::Focus => self
                .window_manager
                .handle_keypress_focus(command.args.clone()),
            WmCommands::Resize => info!("Resize"),
            WmCommands::Quit => process::exit(0),
            WmCommands::Kill => self.window_manager.handle_keypress_kill(),
            WmCommands::Layout => self
                .window_manager
                .handle_keypress_layout(command.args.clone()),
            WmCommands::Restart => self.window_manager.restart = true,
            WmCommands::GoToWorkspace => self
                .window_manager
                .handle_keypress_go_to_workspace(command.args.clone()),
            WmCommands::MoveToWorkspace => self
                .window_manager
                .handle_move_to_workspace(command.args.clone()),
            WmCommands::MoveToWorkspaceAndFollow => self
                .window_manager
                .handle_move_to_workspace_follow(command.args.clone()),
            WmCommands::QuitWorkspace => self.window_manager.handle_quit_workspace(),
            WmCommands::Exec => exec_user_command(&command.args),
            WmCommands::Fullscreen => self.window_manager.handle_keypress_fullscreen(),
        }
    }
}
