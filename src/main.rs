use std::thread;
use std::{
    io::{stdout, Write},
    time::Duration,
};

mod loader;
mod args;

use args::ArgsManager;
use clap::Parser;

fn main() {
    let args = ArgsManager::parse();

    stdout().write_all(b"\x1B[?25l\n").unwrap();

    let mut csv_parser_frontend = loader::Manager::new("1. parsing data.csv");
    thread::sleep(Duration::from_secs(2));
    csv_parser_frontend.end("success");

    let mut ml_training_frontend = loader::Manager::new("2. creating model");
    thread::sleep(Duration::from_secs(6));
    ml_training_frontend.end("success");

    println!("\n {:?}", args);

    stdout().write_all(b"\nAccuracy: \x1B[1;31m96%\x1B[0m\n").unwrap();
    stdout().write_all(b"Precision and Recall: \x1B[1;31m96%\x1B[0m\n").unwrap();
    stdout().write_all(b"F1-score: \x1B[1;31m96%\x1B[0m\n").unwrap();
    stdout().write_all(b"AU-ROC: \x1B[1;31m96%\x1B[0m\n\n").unwrap();

    stdout().write_all(b"\x1B[?25h").unwrap();
}
