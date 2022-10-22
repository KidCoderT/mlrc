use std::{
    error::Error,
    fs::File,
    io::Read,
    mem::discriminant,
    path::PathBuf
};

use csv::ReaderBuilder;
use csv::Reader;

#[derive(Debug, PartialEq, Clone)]
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
    file.read_to_string(&mut raw_data)
        .expect("Error while reading file");

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

    let mut old_record: Vec<Value> = Vec::new();

    for (row, result) in reader.records().enumerate() {
        let record = result.unwrap();
        for (column, field) in record.iter().enumerate() {
            let mut value = Value::String(field.to_string());

            let is_int = field.parse::<i64>();
            let is_float = field.parse::<f64>();

            if is_int.is_ok() {
                value = Value::Integer(is_int.unwrap());
            } else if is_float.is_ok() {
                value = Value::Float(is_float.unwrap());
            }

            if row > 0 && discriminant(&old_record[column]) != discriminant(&value) {
                println!(
                    "the value: {:?}, the old record: {:?}",
                    value, old_record[column]
                );
                return Err(From::from(
                    "the file data has some inconsistent data types!",
                ));
            }

            if old_record.len() == column {
                old_record.push(value.clone());
            } else {
                old_record[column] = value.clone();
            }

            data[column].push(value)
        }
    }

    Ok(data)
}
