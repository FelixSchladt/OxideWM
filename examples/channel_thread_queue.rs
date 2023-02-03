use std::fs;
use std::thread;
//use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

fn get_filenames(folder: &str) -> Vec<String> {
    let paths = fs::read_dir(folder).unwrap();
    let mut files = Vec::new();
    for path in paths {
        files.push(String::from(path.unwrap().path().to_str().unwrap()))
    }
    return files;
}

//this converts files into "events" for testing
//Only returns one event per call
fn get_event(folder: &str) -> Option<String> {
    let mut files = get_filenames(folder);
    let event = files.pop();
    if !event.is_none() {
        fs::remove_file(event.as_ref().unwrap().as_str()).unwrap();
    }
    return event;
}

fn thread1(tx: Sender<String>) {
    loop {
        let event = get_event("thread_1");
        if !event.is_none() {
            tx.send(event.unwrap()).unwrap();
        }
        thread::sleep(Duration::from_secs(2));
    }
}

//Yes I know its code duplication BUT in future these parts will behave differently!
fn thread2(tx: Sender<String>) {
    loop {
        let event = get_event("thread_2");
        if !event.is_none() {
            tx.send(event.unwrap()).unwrap();
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn main() {
    //let mut arc_vec = Arc::new(Vec::new());

    let (tx, rx) = channel();

    let ctx = tx.clone();
    thread::spawn(move || thread1(ctx));

    thread::spawn(move || thread2(tx));

    loop {
        println!("Recv: {}", rx.recv().expect("RECEVEIVE FAILED"));
    }
}
