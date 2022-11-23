use std::process::exit;

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::*;

use x11rb::protocol::ErrorKind;
use x11rb::errors::ReplyError;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn become_wm<C: Connection>(conn: &C, screen: &Screen) -> Result<(), ReplyError> {
    // Try to become the window manager. This causes an error if there is already another WM.
    let change = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);
    let res = conn.change_window_attributes(screen.root, &change)?.check();
    if let Err(ReplyError::X11Error(ref error)) = res {
        if error.error_kind == ErrorKind::Access {
            eprintln!("Another WM is already running.");
            exit(1);
        } else {
            res
        }
    } else {
        res
    }
}

fn manage_window<C: Connection>(
    conn: &C,
    screen_num:usize,
    win: Window,
    geom: &GetGeometryReply,
){
    println!("Managing window {:?}", win);
    let screen = &conn.setup().roots[screen_num];

    let frame_win = conn.generate_id().unwrap();
    let win_aux = CreateWindowAux::new()
        .event_mask(
            EventMask::EXPOSURE
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::BUTTON_PRESS
                | EventMask::BUTTON_RELEASE
                | EventMask::POINTER_MOTION
                | EventMask::ENTER_WINDOW,
        )
        .background_pixel(screen.white_pixel);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        frame_win,
        screen.root,
        geom.x,
        geom.y,
        geom.width,
        geom.height,
        1,
        WindowClass::INPUT_OUTPUT,
        0,
        &win_aux,
    ).unwrap();

    conn.grab_server().unwrap();
    conn.change_save_set(SetMode::INSERT, win).unwrap();
    let cookie = conn
        .reparent_window(win, frame_win, 0, 0).unwrap();
    conn.map_window(win).unwrap();
    conn.map_window(frame_win).unwrap();
    conn.ungrab_server().unwrap();

    
}

fn handle_map_request<C: Connection>(
    conn: &C,
    screen_num:usize,
    event: &MapRequestEvent
){
    let configure_window_aux = ConfigureWindowAux::default()
        .height(600)
        .width(800);
    
    conn.configure_window(event.window, &configure_window_aux).unwrap();

    manage_window(
        conn,
        screen_num,
        event.window,
        &conn.get_geometry(event.window).unwrap().reply().unwrap(),
    )
}

fn handle_event<C: Connection>(
    conn: &C,
    screen_num:usize,
    event: &Event){
    println!("Got event {:?}", event);
    match event {
        Event::UnmapNotify(_event) => println!("unmap"),
        Event::ConfigureRequest(_event) => println!("configure request"),
        Event::MapRequest(event) => handle_map_request(conn, screen_num, event),
        Event::Expose(_event) => println!("expose"),
        Event::EnterNotify(_event) => println!("enter"),
        Event::ButtonPress(_event) => println!("button press"),
        Event::ButtonRelease(_event) => println!("button release"),
        Event::MotionNotify(_event) => println!("motion notify"),
        _ => {}
    }
}

fn main() {
    let (conn, screen_num) = x11rb::rust_connection::RustConnection::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];
    become_wm(&conn, screen).unwrap();
    println!("connection established");

    loop {
        conn.flush().unwrap();

        let event = conn.wait_for_event().unwrap();
        let mut event_option = Some(event);
        while let Some(event) = event_option {
            handle_event(&conn,screen_num,&event);
            event_option = conn.poll_for_event().unwrap();
        }
    }
}
