// This code is derived from https://github.com/psychon/x11rb/blob/c3894c092101a16cedf4c45e487652946a3c4284/cairo-example/src/main.rs
mod config;
mod xcb_visualtype;

use log::info;
//use cairo::glib::subclass::shared::RefCounted;
use x11rb::atom_manager;
use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xproto::{ConnectionExt as _, *};
use x11rb::wrapper::ConnectionExt;
use x11rb::xcb_ffi::XCBConnection;

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::xcb_visualtype::{choose_visual, find_xcb_visualtype};
use oxideipc;
use oxideipc::state::OxideState;

use crate::config::Config;

#[cfg(debug_assertions)]
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    encode::pattern::PatternEncoder,
};

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

pub enum EventType {
    X11rbEvent(x11rb::protocol::Event),
    OxideState(OxideState),
}

#[derive(Debug)]
struct OxideBar {
    conn: Arc<XCBConnection>,
    window: Window,
    atoms: AtomCollection,
    screen: u32,
    visual_id: u32,
    config: Config,
    depth: u8,
    cairo_surface: Option<cairo::XCBSurface>,
    composite_mgr: bool,
}

impl OxideBar {
    pub fn new(conn: Arc<XCBConnection>, config: Config, screen_num: usize) -> OxideBar {
        let window = conn.generate_id().unwrap();
        let screen = conn.setup().roots[screen_num].root;
        let (depth, visual_id) = choose_visual(conn.as_ref(), screen_num).unwrap();
        info!("Using visual {:#x} with depth {}", visual_id, depth);
        let atoms = AtomCollection::new(conn.as_ref()).unwrap().reply().unwrap();
        let cairo_surface = None;
        let composite_mgr = false;

        let mut bar = OxideBar {
            conn,
            window,
            atoms,
            screen,
            visual_id,
            config,
            depth,
            cairo_surface,
            composite_mgr,
        };
        bar.composite_manager_running(screen_num);
        bar.create_window(screen_num).unwrap();
        bar.create_cairo_surface();
        bar.draw(oxideipc::get_state_struct());

        return bar;
    }

    fn composite_manager_running(&mut self, screen_num: usize) {
        let atom = format!("_NET_WM_CM_S{}", screen_num);
        let atom = self
            .conn
            .intern_atom(false, atom.as_bytes())
            .unwrap()
            .reply()
            .unwrap()
            .atom;
        let owner = self
            .conn
            .get_selection_owner(atom)
            .unwrap()
            .reply()
            .unwrap();
        self.composite_mgr = owner.owner != x11rb::NONE;
    }

    fn create_window(&mut self, screen_num: usize) -> Result<(), ReplyOrIdError> {
        let colormap = self.conn.generate_id().unwrap();
        self.conn
            .create_colormap(ColormapAlloc::NONE, colormap, self.screen, self.visual_id)
            .unwrap();
        let screen = &self.conn.setup().roots[screen_num];
        let win_aux = CreateWindowAux::new()
            .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
            .background_pixel(x11rb::NONE)
            .border_pixel(screen.white_pixel)
            //.background_pixel(screen.white_pixel)
            .colormap(colormap);

        info!("Visual id: {:#x}", self.visual_id);

        self.conn.create_window(
            self.depth,
            self.window,
            self.screen,
            0,
            0,
            self.config.width,
            self.config.height,
            0,
            WindowClass::INPUT_OUTPUT,
            self.visual_id,
            &win_aux,
        )?;

        let title = "OxideBar";
        self.conn.change_property8(
            PropMode::REPLACE,
            self.window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            title.as_bytes(),
        )?;
        self.conn.change_property8(
            PropMode::REPLACE,
            self.window,
            self.atoms._NET_WM_NAME,
            self.atoms.UTF8_STRING,
            title.as_bytes(),
        )?;
        self.conn.change_property32(
            PropMode::REPLACE,
            self.window,
            self.atoms.WM_PROTOCOLS,
            AtomEnum::ATOM,
            &[self.atoms.WM_DELETE_WINDOW],
        )?;
        self.conn.change_property8(
            PropMode::REPLACE,
            self.window,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            b"oxide-bar\0oxide-bar\0",
        )?;
        self.conn.change_property32(
            PropMode::REPLACE,
            self.window,
            self.atoms._NET_WM_WINDOW_TYPE,
            AtomEnum::ATOM,
            &[self.atoms._NET_WM_WINDOW_TYPE_DOCK],
        )?;

        self.conn.map_window(self.window)?;
        self.conn.as_ref().flush()?;
        Ok(())
    }

    fn create_cairo_surface(&mut self) {
        let mut visual = find_xcb_visualtype(self.conn.as_ref(), self.visual_id).unwrap();
        let cairo_conn =
            unsafe { cairo::XCBConnection::from_raw_none(self.conn.get_raw_xcb_connection() as _) };
        let visual = unsafe { cairo::XCBVisualType::from_raw_none(&mut visual as *mut _ as _) };
        self.cairo_surface = Some(
            cairo::XCBSurface::create(
                &cairo_conn,
                &cairo::XCBDrawable(self.window),
                &visual,
                self.config.width.into(),
                self.config.height.into(),
            )
            .unwrap(),
        );
    }

    fn draw(&mut self, state: OxideState) {
        let cr = cairo::Context::new(self.cairo_surface.as_ref().unwrap())
            .expect("failed to create cairo context");
        if self.composite_mgr {
            cr.set_operator(cairo::Operator::Source);
            cr.set_source_rgba(0.9, 1.0, 0.9, 0.5);
        } else {
            cr.set_source_rgb(0.9, 1.0, 0.9);
        }
        cr.paint().unwrap();
        if self.composite_mgr {
            cr.set_operator(cairo::Operator::Over);
        }
        cr.show_text("Hi there").unwrap();

        let ws_vec = state.get_workspace_list(self.screen);
        info!("ws_vec: {:?}", ws_vec);

        let active_ws = state.get_active_workspace(self.screen);
        info!("active workspace: {}", active_ws);

        let mut x = 10.0;
        for ws in ws_vec {
            if ws == active_ws {
                cr.set_source_rgb(0.0, 0.0, 0.0);
            } else {
                cr.set_source_rgb(0.5, 0.5, 0.5);
            }
            cr.move_to(x, 20.0);
            cr.set_font_size(15.0);
            cr.show_text(&ws.to_string()).unwrap();
            x += 20.0;
        }
        self.cairo_surface.as_ref().unwrap().flush();
    }

    pub fn handle_oxide_state_event(&mut self, state: OxideState) {
        info!("oxide state event");
        self.draw(state);
    }

    pub fn handle_x_event(&mut self, event: x11rb::protocol::Event) {
        match event {
            x11rb::protocol::Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32
                    && event.window == self.window
                    && data[0] == self.atoms.WM_DELETE_WINDOW
                {
                    info!("Oxide-bar exiting");
                    std::process::exit(0);
                }
            }
            x11rb::protocol::Event::Error(error) => {
                info!("Error: {:?}", error);
            }
            _ => {}
        }
    }
}

fn get_x11rb_events(
    connection: Arc<XCBConnection>,
    event_sender_mutex: Arc<Mutex<Sender<EventType>>>,
) {
    loop {
        match connection.wait_for_event() {
            Ok(event) => {
                event_sender_mutex
                    .lock()
                    .unwrap()
                    .send(EventType::X11rbEvent(event))
                    .unwrap();
            }
            Err(error) => info!("Error: {}", error),
        }
    }
}

fn get_state(event_sender_mutex: Arc<Mutex<Sender<EventType>>>) {
    let (event_sender, event_receiver) = channel::<OxideState>();

    let ipc_event_sender_mutex = Arc::new(Mutex::new(event_sender));

    thread::spawn(move || {
        oxideipc::state_signal_channel(ipc_event_sender_mutex);
    });

    loop {
        if let Ok(event) = event_receiver.recv() {
            event_sender_mutex
                .lock()
                .unwrap()
                .send(EventType::OxideState(event))
                .unwrap();
        }
    }
}

#[cfg(debug_assertions)]
fn get_log_file_appender() -> RollingFileAppender {
    let log_file_pattern = format!("{}{}{{}}.{}", "log/bar/", "oxidebar", "log");
    let log_file = format!("{}{}.{}", "log/bar/", "oxidebar", "log");

    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        .build(log_file_pattern.as_str(), window_size)
        .unwrap();

    let size_limit = 5 * u64::pow(2, 20); // 5MB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        )))
        .build(log_file, Box::new(compound_policy))
        .unwrap()
}

#[cfg(debug_assertions)]
fn init_logger() {
    use log::LevelFilter;
    use log4rs::{
        append::console::ConsoleAppender,
        config::{Appender, Root},
    };

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - [{h({l})}]: {m}{n}",
        )))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
        .appender(Appender::builder().build("logfile", Box::new(get_log_file_appender())))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();
    log4rs::init_config(config).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    init_logger();

    let (connection, screen_num) = XCBConnection::connect(None)?;
    let conn = Arc::new(connection);

    let screen = &conn.setup().roots[screen_num];

    let config: Config = Config::new(screen.width_in_pixels);

    let mut bar = OxideBar::new(conn.clone(), config, screen_num);

    let (event_sender, event_receiver) = channel::<EventType>();

    let event_sender_mutex = Arc::new(Mutex::new(event_sender));
    let event_receiver_mutex = Arc::new(Mutex::new(event_receiver));

    let event_sender_clone = event_sender_mutex.clone();
    thread::spawn(move || get_state(event_sender_clone));
    let conn_clone = conn.clone();
    thread::spawn(move || get_x11rb_events(conn_clone, event_sender_mutex));

    loop {
        conn.flush().ok();
        if let Ok(event_type) = event_receiver_mutex.lock().unwrap().recv() {
            match event_type {
                EventType::X11rbEvent(event) => bar.handle_x_event(event),
                EventType::OxideState(event) => bar.handle_oxide_state_event(event),
            }
        }
    }
}
