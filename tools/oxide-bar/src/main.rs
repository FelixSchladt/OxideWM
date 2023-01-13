use gtk::{Application, ApplicationWindow};
use gtk::Button;
use gtk::Orientation;
//use gdk_x11::x11::xlib::{PropModeReplace, XChangeProperty, XInternAtom, XA_ATOM};
//use gdk_x11::x11::xlib;
use gdk_x11::x11::xlib::{PropModeReplace, XChangeProperty, XA_ATOM};
use gdk_x11_sys::xlib::XInternAtom;
use gdk_x11::x11::xlib;
use std::ffi::CString;
use gdk_x11::X11Display;
use gdk_x11::X11Surface;

use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;
use gtk::prelude::ButtonExt;
use gtk::prelude::BoxExt;
use gtk::prelude::GtkWindowExt;

use gtk::prelude::WidgetExt;

use gtk::prelude::NativeExt;

use gtk::prelude::Cast;
use gdk::prelude::SurfaceExt;



use oxideipc;
use oxideipc::state::*;

const APP_ID: &str = "org.oxide.oxide-bar";


fn main() {
    let app = Application::builder().application_id(APP_ID).build();


    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let mut workspace_button: Vec<gtk::Button> = Vec::new();
    let state = oxideipc::get_state_struct();
    let screen = state.focused_screen; //Todo get rootid of screen where status bar run

    for (i, ws) in state.get_workspaces(state.focused_screen).iter().enumerate() {
        let button = Button::builder()
                .label(&ws.index.to_string())
                .margin_top(2)
                .margin_bottom(2)
                .margin_start(2)
                .margin_end(2)
                .build();

        button.connect_clicked(move |button| {
            //let label = format!("{}", state.workspaces[i].index.clone().to_string());
            let label = "0";
            button.set_label(&label);
            //switch_workspace(i);
            oxideipc::next_workspace();

        });
        workspace_button.push(button);
    }

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    for button in workspace_button {
        gtk_box.append(&button);
    }

    //build application with all button in the workspace_button vector
    let mut window = gtk::Window::builder()
        .application(app)
        .title("Oxide Bar")
        .child(&gtk_box)
        .build();

    let prop_values: Vec<&str> = vec!["_NET_WM_WINDOW_TYPE_DOCK"];

    let toplevels = gtk::Window::list_toplevels();
    println!("toplevels: {:?}", toplevels);
    let top = toplevels[0].clone();
    let display = top.display();
    




    // Present window
    window.present();

    set_window_props(&mut window, "_NET_WM_WINDOW_TYPE", &prop_values);
}


//https://stackoverflow.com/questions/68476172/how-do-you-set-x11-window-hints-using-gtk4-rs


fn set_window_props(window: &gtk::Window, prop_name: &str, prop_values: &Vec<&str>) {
    let display = window.display();
    let surface = window.surface().unwrap();
    let prop_name_cstr = CString::new(prop_name).unwrap();
    let prop_values_cstr: Vec<CString> = prop_values
        .iter()
        .map(|val| CString::new(*val).unwrap())
        .collect();
    unsafe {
        let xid: xlib::Window = surface.unsafe_cast::<X11Surface>().xid();
        let xdisplay: *mut xlib::Display = display.unsafe_cast::<X11Display>().xdisplay();
        let prop_name_atom = XInternAtom(xdisplay, prop_name_cstr.as_ptr(), xlib::False);
        let mut prop_values_atom: Vec<u64> = prop_values_cstr
            .into_iter()
            .map(|cstr| XInternAtom(xdisplay, cstr.as_ptr(), xlib::False))
            .collect();
        let num_values = prop_values_atom.len();
        let prop_values_c = prop_values_atom.as_mut_ptr();
        XChangeProperty(
            xdisplay,
            xid,
            prop_name_atom,
            XA_ATOM,
            32,
            PropModeReplace,
            prop_values_c as *const u8,
            num_values as i32,
        );
    }
}

