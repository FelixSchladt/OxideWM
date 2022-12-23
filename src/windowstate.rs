use x11rb::protocol::xproto::Window;

pub struct WindowState {
    //window: Window,
    title: String,
    visible: bool,
    focused: bool,
    urgent:  bool,
    titlebar_height: u16,

    frame:    Window,
    titlebar: Window,
    window:   Window,
}
