use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::io::BufReader;
use std::path::Path;

type IoResult<T> = Result<T, io::Error>;

pub fn read(day: u32) -> IoResult<Vec<String>> {
    let path_name = format!("input/{:02}.txt", day);
    let path = Path::new(path_name.as_str());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn read_all(day: u32) -> IoResult<String> {
    let path_name = format!("input/{:02}.txt", day);
    let path = Path::new(path_name.as_str());
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut output = String::new();
    reader.read_to_string(&mut output)?;

    Ok(output)
}