use chrono::Local;

//get_time_fomat("%d %b %H:%M"));
fn get_time_fomat(time_format: &str) -> String {
    let date = Local::now();
    date.format(time_format).to_string()
}



