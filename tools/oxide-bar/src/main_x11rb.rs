use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{
    protocol::{
        ErrorKind,
        Event
    },
    protocol::xproto::{
        ChangeWindowAttributesAux,
        Screen,
        MapRequestEvent, 
        UnmapNotifyEvent, 
        LeaveNotifyEvent, 
        EnterNotifyEvent, 
        EventMask, 
        GrabMode, 
        ModMask,
        AtomEnum,
        ConnectionExt as _,
        PropMode,
        CreateWindowAux,
    },
    rust_connection::{
        ConnectionError,
        RustConnection,
        ReplyError
    }
};
use x11rb::wrapper::ConnectionExt as _;



use oxideipc;
use oxideipc::state::*;
use oxidewm::atom::Atom;


struct OxideBar {
    connection: RustConnection,
    screen: Screen,
    window: u32,
    //state: OxideState,
}

impl OxideBar {
    pub fn new() -> OxideBar {
        let (connection, screen_num) = RustConnection::connect(None).unwrap();
        let screen = connection.setup().roots[screen_num].clone();
        let window = connection.generate_id().unwrap();
        //let state = oxideipc::get_state_struct();
        let oxide_bar = OxideBar {
            connection,
            screen,
            window,
        };
        oxide_bar.setup_window();
        oxide_bar.setup_props();
        oxide_bar.connection.flush().unwrap();
        oxide_bar

    }

    fn setup_window(&self) {
        let window = self.window;
        let screen = &self.screen;
        let connection = &self.connection;
        let mask = CreateWindowAux::new()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY)
            .background_pixel(screen.white_pixel);
        connection
            .create_window(
                screen.root_depth,
                window,
                screen.root,
                0,
                0,
                screen.width_in_pixels,
                20,
                0,
                x11rb::protocol::xproto::WindowClass::INPUT_OUTPUT,
                screen.root_visual,
                &mask,
            )
            .unwrap();
        connection
            .map_window(window)
            .unwrap();
    }

    fn setup_props(&self) {
        let atom_intern = self.connection.intern_atom(false, "_NET_WM_WINDOW_TYPE".as_bytes()).unwrap().reply().unwrap().atom;
        let atom_intern_prop = self.connection.intern_atom(false, "_NET_WM_WINDOW_TYPE_DOCK".as_bytes()).unwrap().reply().unwrap().atom;
        let atom_intern_prop_slice: Vec<u32> = vec![atom_intern_prop];
        println!("atom_intern_prop_slice: {:?}", atom_intern_prop_slice);

        self.connection.change_property32(
            PropMode::REPLACE,
            self.window,
            atom_intern,
            AtomEnum::ATOM,
            &atom_intern_prop_slice,
            );
        self.connection.flush().unwrap();

    }

    /*
    pub fn update_state(&mut self) {
        self.state = oxideipc::get_state_struct();
    }*/
}


fn main() {
    let bar = OxideBar::new();

    loop { bar.connection.wait_for_event().unwrap(); }

}
