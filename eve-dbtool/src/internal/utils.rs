use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
};

fn get_gzip_sql_content(file_name: &str) -> Result<String, Error> {
    let file = File::open(&file_name).unwrap();
    let file = BufReader::new(file);
    let mut file = GzDecoder::new(file);
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        return Err(err);
    }
    Ok(content)
}

fn get_plain_sql_content(file_name: &str) -> Result<String, Error> {
    let file = File::open(&file_name).unwrap();
    let mut file = BufReader::new(file);
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        return Err(err);
    }
    Ok(content)
}

pub fn get_sql_content(file_name: &str) -> Result<String, Error> {
    if file_name.ends_with(".gz") {
        return get_gzip_sql_content(file_name);
    }
    get_plain_sql_content(file_name)
}
