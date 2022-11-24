use std::process::exit;
use std::error::Error;

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
    ChangeWindowAttributesAux,
    EventMask,
};

pub struct WindowManager {
    pub connection: RustConnection,
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
                eprintln!("\x1b[31m\x1b[1mError:\x1b0m Access to X11 Client Api denied!");
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
