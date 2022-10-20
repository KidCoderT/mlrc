use std::thread;
use std::{
    io::{stdout, Write},
    time::Duration,
};

mod loader;

fn main() {
    stdout().write_all(b"\x1B[2J\x1B[1;0H\x1B[?25l").expect("error occurred!");

    let mut csv_parser_frontend = loader::Manager::new("1. parsing data.csv");
    thread::sleep(Duration::from_secs(2));
    csv_parser_frontend.end("success");

    let mut ml_training_frontend = loader::Manager::new("2. creating model");
    thread::sleep(Duration::from_secs(6));
    ml_training_frontend.end("success");

    stdout().write_all(b"\x1B[?25h").expect("error occurred!");
}
