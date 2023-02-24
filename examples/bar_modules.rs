extern crate chrono;

use chrono::Local;

fn get_time_fomat(time_format: &str) -> String {
    let date = Local::now();
    date.format(time_format).to_string()
}

fn main() {
    println!("{}", get_time_fomat("%d %b %H:%M"));
}
