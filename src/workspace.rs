use super::windowstate::WindowState;

pub enum Layout {
    TILING,
    //Different layout modes and better names wanted C:
}

pub struct Workspace {
    name: String,
    index: u16,
    visible: bool,
    focused: bool,
    urgent: bool,
    windows: Vec<WindowState>,
    layout: Layout,
}

impl Workspace {
    pub fn show() { panic!("Not implemented"); }
    pub fn hide() { panic!("Not implemented"); }
}
