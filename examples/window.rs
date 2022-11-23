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
    let frame_id = connection.generate_id()?;
    let titlebar_id = connection.generate_id()?; 
    let window_id = connection.generate_id()?;
    let titlebar_height: i16 = 10;

    connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        frame_id,
        screen.root,
        x,
        y,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;

    connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        titlebar_id,
        frame_id,
        border_width as i16,
        border_width as i16,
        width - 2*border_width,
        titlebar_height as u16,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;

    connection.create_window(
        COPY_DEPTH_FROM_PARENT,
        window_id,
        frame_id,
        border_width as i16,
        titlebar_height + border_width as i16,
        width - 2*border_width,
        height- titlebar_height as u16 - 2*border_width,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;

    connection.map_window(frame_id)?;
    connection.map_window(window_id)?;
    connection.flush()?;

    loop {
        println!("Event: {:?}", connection.wait_for_event().unwrap());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    create_example(
        20,   //x
        20,   //y
        300, //width
        400, //height
        5,  //border_width
    )
}
