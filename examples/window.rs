use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::protocol::xproto::*;
use std::error::Error;

pub fn create_example(
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
    ) -> Result<(), Box<dyn Error>>{
    let (connection, screen_index) = x11rb::connect(None)?;
    let screen = &connection.setup().roots[screen_index];
    let window_id = connection.generate_id()?;

    connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        window_id,
        screen.root,
        x,
        y,
        width,
        height,
        border_width,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    ).unwrap();

    connection.map_window(window_id).unwrap();
    connection.flush()?;

    loop {
        println!("Event: {:?}", connection.wait_for_event().unwrap());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    create_example(
        0,   //x
        0,   //y
        100, //width
        100, //height
        10,  //border_width
    )
}
