use std::error::Error;

mod frames;

fn main() -> Result<(), Box<dyn Error>>{
    frames::create_example(
        0,   //x
        0,   //y
        100, //width
        100, //height
        5    //border_width
    )
}
