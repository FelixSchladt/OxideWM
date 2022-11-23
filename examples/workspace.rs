mod workspace {
    pub enum Layout {
        TILING,
        //Different layout modes and better names wanted C:
    }

    pub struct Workspace {
        // reference to window_manager
        // reference to root window
        windows: Vec,
        layout: Layout,
    };

    impl Workspace {
        pub fn new() -> Workspace { panic!("Not implemented"); }

        pub fn show() { panic!("Not implemented"); }
        pub fn hide() { panic!("Not implemented"); }
        pub fn open_window() { panic!("Not implemented"); }
        pub fn hide_window() { panic!("Not implemented"); }

        fn remap_windows() { panic!("Not implemented"); }
    };
}
