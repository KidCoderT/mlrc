use std::thread;
use std::{
    io::{stdout, Write},
    time::Duration,
};

mod loader;
mod parser;
mod args;

use args::ArgsManager;
use clap::Parser;


fn main() {
    let args = ArgsManager::parse();

    stdout().write_all(b"\x1B[?25l\n").unwrap();

    let mut csv_parser_frontend = loader::Manager::new("1. parsing data.csv");

    thread::sleep(Duration::from_millis(500));
    
    let mut delimiter = b","[0];
    if let Some(ref new_delimiter) = args.delimiter {
        delimiter = new_delimiter.as_bytes()[0];
    }

    let mut csv_data;
    match parser::read_csv_file(&args.data_path, delimiter) {
        Ok(data) => {
            csv_data = data;
        },
        Err(error) => {
            csv_parser_frontend.end("fail");
            panic!("{:?}", error)
        }
    }

    // println!("{:#?}", csv_data);
    
    csv_parser_frontend.end("success");
    
    // let mut ml_training_frontend = loader::Manager::new("2. creating model");
    // thread::sleep(Duration::from_secs(6));
    // ml_training_frontend.end("success");

    // thread::sleep(Duration::from_secs(1));

    // println!("\n{:?}", args);

    // stdout().write_all(b"\n Accuracy: \x1B[1;31m96%\x1B[0m\n").unwrap();
    // stdout().write_all(b" Precision and Recall: \x1B[1;31m96%\x1B[0m\n").unwrap();
    // stdout().write_all(b" F1-score: \x1B[1;31m96%\x1B[0m\n").unwrap();
    // stdout().write_all(b" AU-ROC: \x1B[1;31m96%\x1B[0m\n\n").unwrap();
    
    // println!("model file at: \x1B[4m\x1B[1;34mresult.txt\x1B[0m");

    stdout().write_all(b"\x1B[?25h").unwrap();
}
