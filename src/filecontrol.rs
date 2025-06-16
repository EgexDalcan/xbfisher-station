use std::{fs::{self, OpenOptions}, io::{Error, ErrorKind}};
use std::io::Write;
use chrono::{Datelike, Local, Timelike};

pub fn write_error(error: Error){
    let date: chrono::DateTime<Local> = chrono::offset::Local::now();
    fs::create_dir("./data").unwrap_or_else(|error| {if error.kind() == ErrorKind::AlreadyExists{} else {panic!("Error while creating data directory. Error: {error}")}});
    let file_name = format!("ErrorList");
    // Open file, if no file, create one.
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_name)
        .unwrap_or_else(| error | {
            panic!("Problem writing to the file: {error:?}");
        });
    let date_str = format!("{}_{}_{}_{}:{}", date.month(), date.day(), date.year(), date.hour(), date.minute());
    // Write.
    writeln!(file, "{date_str}: Error while receiving connection: {error}");
}