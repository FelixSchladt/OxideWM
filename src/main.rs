use x11rb::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

//TODO: Config Struct
struct Config {}

struct WindowState {
    window: Window,
    title: String,
    visible: bool,
    focused: bool,
    urgent: bool,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    titlebar_height: u16,
}


struct Workspace {
    name: String,
    index: u16,
    visible: bool,
    focused: bool,
    urgent: bool,
    windows: Vec<WindowState>,
}

struct Monitor {
    index: usize,
    screen: Screen,
    workspaces: Vec<Workspace>,
    width: u16,
    height: u16,
}

struct WindowManager {
    conn: Connection,
    monitor: Vec<MonitorStruct>,
    config: Config,
}
