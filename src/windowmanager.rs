use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::Screen;

pub struct WindowManager {
    connection: RustConnection,
    //config: Config,
}

impl WindowManager {
    pub fn new () -> WindowManager {
        let (connection, _) = RustConnection::connect(None).unwrap();

        WindowManager {
            connection,
        } 
    }

    fn update_root_window_event_masks(&self) {
        for root in elf.connection
                        .setup()
                        .roots {
            self.enable_substructure_redirect(root);
        }
    }

    fn enable_substructure_redirect(&self, root_window: &Screen) {
        
    }
}
