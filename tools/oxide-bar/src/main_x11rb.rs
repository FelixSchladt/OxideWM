use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
/*
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
*/

use x11rb::atom_manager;
use x11rb::connection::Connection;
use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::protocol::render::{self, ConnectionExt as _, PictType};
use x11rb::protocol::xproto::{ConnectionExt as _, *};
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt;
use x11rb::xcb_ffi::XCBConnection;


use oxideipc;
use oxideipc::state::*;
use oxidewm::atom::Atom;

const FONT: &str = "ProFontIIx Nerd Font";

struct OxideBar {
    connection: RustConnection,
    screen: Screen,
    window: u32,
    state: OxideState,
}


/// A rust version of XCB's `xcb_visualtype_t` struct. This is used in a FFI-way.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: u32,
    pub class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

impl From<Visualtype> for xcb_visualtype_t {
    fn from(value: Visualtype) -> xcb_visualtype_t {
        xcb_visualtype_t {
            visual_id: value.visual_id,
            class: value.class.into(),
            bits_per_rgb_value: value.bits_per_rgb_value,
            colormap_entries: value.colormap_entries,
            red_mask: value.red_mask,
            green_mask: value.green_mask,
            blue_mask: value.blue_mask,
            pad0: [0; 4],
        }
    }
}

/// Find a `xcb_visualtype_t` based on its ID number
fn find_xcb_visualtype(conn: &impl Connection, visual_id: u32) -> Option<xcb_visualtype_t> {
    for root in &conn.setup().roots {
        for depth in &root.allowed_depths {
            for visual in &depth.visuals {
                if visual.visual_id == visual_id {
                    return Some((*visual).into());
                }
            }
        }
    }
    None
}

/// Choose a visual to use. This function tries to find a depth=32 visual and falls back to the
/// screen's default visual.
fn choose_visual(conn: &impl Connection, screen_num: usize) -> Result<(u8, Visualid), ReplyError> {
    let depth = 32;
    let screen = &conn.setup().roots[screen_num];

    // Try to use XRender to find a visual with alpha support
    let has_render = conn
        .extension_information(render::X11_EXTENSION_NAME)?
        .is_some();
    if has_render {
        let formats = conn.render_query_pict_formats()?.reply()?;
        // Find the ARGB32 format that must be supported.
        let format = formats
            .formats
            .iter()
            .filter(|info| (info.type_, info.depth) == (PictType::DIRECT, depth))
            .filter(|info| {
                let d = info.direct;
                (d.red_mask, d.green_mask, d.blue_mask, d.alpha_mask) == (0xff, 0xff, 0xff, 0xff)
            })
            .find(|info| {
                let d = info.direct;
                (d.red_shift, d.green_shift, d.blue_shift, d.alpha_shift) == (16, 8, 0, 24)
            });
        if let Some(format) = format {
            // Now we need to find the visual that corresponds to this format
            if let Some(visual) = formats.screens[screen_num]
                .depths
                .iter()
                .flat_map(|d| &d.visuals)
                .find(|v| v.format == format.id)
            {
                return Ok((format.depth, visual.visual));
            }
        }
    }
    Ok((screen.root_depth, screen.root_visual))
}

impl OxideBar {
    pub fn new() -> OxideBar {
        let (connection, screen_num) = RustConnection::connect(None).unwrap();
        let screen = connection.setup().roots[screen_num].clone();
        let window = connection.generate_id().unwrap();
        let state = oxideipc::get_state_struct();
        let oxide_bar = OxideBar {
            connection,
            screen,
            window,
            state,
        };
        oxide_bar.setup_window();
        oxide_bar.setup_props();
        oxide_bar.connection.flush().unwrap();
        oxide_bar

    }

    fn setup_window(&self) {
        let window = self.window;
        let colormap = self.connection.generate_id().unwrap();
        let screen = &self.screen;
        let connection = &self.connection;
        let mask = CreateWindowAux::new()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY)
            .background_pixel(screen.white_pixel);
            .colormap(colormap
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

    pub fn update_state(&mut self) {
        self.state = oxideipc::get_state_struct();
    }
}


fn main() {
    let bar = OxideBar::new();

    loop { bar.connection.wait_for_event().unwrap(); }

}
