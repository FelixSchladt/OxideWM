use std::process::exit;
use std::error::Error;

use super::workspace::Workspace;
use super::windowstate::WindowState;

use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::ErrorKind;
use x11rb::errors::{
    ReplyError,
    ConnectionError,
};
use x11rb::protocol::xproto::{
    ConnectionExt,
    Screen,
    Window,
    ChangeWindowAttributesAux,
    ConfigureWindowAux,
    CreateWindowAux,
    EventMask,
};

pub struct WindowManager {
    pub connection: RustConnection,
    active_monitor_id: u16,
    //pub monitors: HashMap<u16, Vec<Workspace>>,
    //config: Config,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let (connection, screen_index) = RustConnection::connect(None).unwrap();
        let manager = WindowManager {
            connection,
        };

        manager.update_root_window_event_masks();

        manager
    }

    pub fn map_window(window: Window, monitor_id: u16, workspace_id: u16) {
         
    }

    pub fn remap(monitor_id: u16, workspace_id: u16) {
        //TODO implement method parameters
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
