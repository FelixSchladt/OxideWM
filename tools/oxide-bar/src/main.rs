// This code is derived from https://github.com/psychon/x11rb/blob/c3894c092101a16cedf4c45e487652946a3c4284/cairo-example/src/main.rs
//
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

// A collection of the atoms we will need.
atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DOCK,
    }
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

/// Check if a composite manager is running
fn composite_manager_running(
    conn: &impl Connection,
    screen_num: usize,
) -> Result<bool, ReplyError> {
    let atom = format!("_NET_WM_CM_S{}", screen_num);
    let atom = conn.intern_atom(false, atom.as_bytes())?.reply()?.atom;
    let owner = conn.get_selection_owner(atom)?.reply()?;
    Ok(owner.owner != x11rb::NONE)
}

/// Create a window for us.
fn create_window<C>(
    conn: &C,
    screen: &x11rb::protocol::xproto::Screen,
    atoms: &AtomCollection,
    (width, height): (u16, u16),
    depth: u8,
    visual_id: Visualid,
) -> Result<Window, ReplyOrIdError>
where
    C: Connection,
{
    let window = conn.generate_id()?;
    let colormap = conn.generate_id()?;
    conn.create_colormap(ColormapAlloc::NONE, colormap, screen.root, visual_id)?;
    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
        .background_pixel(x11rb::NONE)
        .border_pixel(screen.black_pixel)
        .colormap(colormap);
    conn.create_window(
        depth,
        window,
        screen.root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        visual_id,
        &win_aux,
    )?;

    let title = "Simple Window";
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )?;
    conn.change_property8(
        PropMode::REPLACE,
        window,
        atoms._NET_WM_NAME,
        atoms.UTF8_STRING,
        title.as_bytes(),
    )?;
    conn.change_property32(
        PropMode::REPLACE,
        window,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_CLASS,
        AtomEnum::STRING,
        b"simple_window\0simple_window\0",
    )?;
    conn.change_property32(
        PropMode::REPLACE,
        window,
        atoms._NET_WM_WINDOW_TYPE,
        AtomEnum::ATOM,
        &[atoms._NET_WM_WINDOW_TYPE_DOCK],
    )?;

    conn.map_window(window)?;
    Ok(window)
}

/// Draw the window content
fn do_draw(
    cr: &cairo::Context,
    (width, height): (f64, f64),
    transparency: bool,
    screen_num: u32,
) -> Result<(), cairo::Error> {
    // Draw a background
    if transparency {
        cr.set_operator(cairo::Operator::Source);
        cr.set_source_rgba(0.9, 1.0, 0.9, 0.5);
    } else {
        cr.set_source_rgb(0.9, 1.0, 0.9);
    }
    cr.paint()?;
    if transparency {
        cr.set_operator(cairo::Operator::Over);
    }

    let ws_vec =  oxideipc::get_state_struct().workspace_tuple(screen_num);
    println!("ws_vec: {:?}", ws_vec);

    let mut x = 10.0;
    for (visible, ws) in ws_vec {
        if visible {
            cr.set_source_rgb(0.0, 0.0, 0.0);
        } else {
            cr.set_source_rgb(0.5, 0.5, 0.5);
        }
        cr.move_to(x, 20.0);
        cr.set_font_size(15.0);
        cr.show_text(&ws)?;
        x += 20.0;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = XCBConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let screen_id = screen.root;
    println!("screen_num: {}", screen_num);
    let atoms = AtomCollection::new(&conn)?.reply()?;
    let (mut width, mut height) = (100, 100);
    let (depth, visualid) = choose_visual(&conn, screen_num)?;
    println!("Using visual {:#x} with depth {}", visualid, depth);

    // Check if a composite manager is running. In a real application, we should also react to a
    // composite manager starting/stopping at runtime.
    let transparency = composite_manager_running(&conn, screen_num)?;
    println!(
        "Composite manager running / working transparency: {:?}",
        transparency
    );

    let window = create_window(&conn, screen, &atoms, (width, height), depth, visualid)?;

    // Here comes all the interaction between cairo and x11rb:
    let mut visual = find_xcb_visualtype(&conn, visualid).unwrap();
    // SAFETY: cairo-rs just passes the pointer to C code and C code uses the xcb_connection_t, so
    // "nothing really" happens here, except that the borrow checked cannot check the lifetimes.
    let cairo_conn = unsafe { cairo::XCBConnection::from_raw_none(conn.get_raw_xcb_connection() as _) };
    let visual = unsafe { cairo::XCBVisualType::from_raw_none(&mut visual as *mut _ as _) };
    let surface = cairo::XCBSurface::create(
        &cairo_conn,
        &cairo::XCBDrawable(window),
        &visual,
        width.into(),
        height.into(),
    ).unwrap();

    loop {
        conn.flush()?;
        let result = conn.poll_for_event();
        let mut need_redraw = false;
        if let Ok(Some(event)) = result {
            println!("{:?})", event);
            match event {
                Event::Expose(_) => {
                    need_redraw = true;
                }
                Event::ConfigureNotify(event) => {
                    width = event.width;
                    height = event.height;
                    surface.set_size(width as _, height as _).unwrap();
                    need_redraw = true;
                }
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32
                        && event.window == window
                        && data[0] == atoms.WM_DELETE_WINDOW
                    {
                        println!("Window was asked to close");
                        return Ok(());
                    }
                }
                Event::Error(_) => println!("Got an unexpected error"),
                _ => println!("Got an unknown event"),
            }
        }
        let cr = cairo::Context::new(&surface).expect("failed to create cairo context");
        do_draw(&cr, (width as _, height as _), transparency, screen_id).expect("failed to draw");
        surface.flush();
        //sleep for 100ms
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
