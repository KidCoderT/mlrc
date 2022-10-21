use std::path::PathBuf;
use std::{
    io::Read,
    fs::File,
    error::Error
};

use csv::ReaderBuilder;
use csv::Reader;
use csv;

#[derive(Debug)]
pub enum Value {
  Integer(i64),
  String(String),
  Float(f64),
}

fn get_column_count<R: Read>(reader: &mut Reader<R>) -> Option<usize> {
    reader
        .records()
        .peekable()
        .peek()
        .and_then(|o| -> Option<_> { Some(o.as_ref().ok()?.len()) })
}

pub fn read_csv_file(path: &PathBuf, delimiter: u8) -> Result<Vec<Vec<Value>>, Box<dyn Error>> {
    let mut raw_data = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut raw_data).expect("Error while reading file");

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(raw_data.as_bytes());
        
    let size: usize;
    
    if let Some(count) = get_column_count(&mut reader) {
        size = count;
    } else {
        return Err(From::from("the file could not be read!"));
    }

    let mut data = Vec::new();
    
    for _ in 0..size {
        data.push(Vec::new());
    }
        
    for result in reader.records() {
        let record = result.unwrap();
        for (i, field) in record.iter().enumerate() {
            let mut value = Value::String(field.to_string());

            let is_int = field.parse::<i64>();
            let is_float = field.parse::<f64>();
            
            if !is_int.is_err() {
                value = Value::Integer(is_int.unwrap());
            } else if!is_float.is_err() {
                value = Value::Float(is_float.unwrap());
            }
            
            data[i].push(value)
        }
    }

    // println!("{}", data.len());
    // for i in 0..data.len() {
    //     println!("{}, {:?}", data[i].len(), data[i][0]);
    // }

    // todo: add data checks
    

    Ok(data)
}
