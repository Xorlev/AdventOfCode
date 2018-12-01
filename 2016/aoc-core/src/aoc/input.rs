use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

type IoResult<T> = Result<T, io::Error>;

pub fn read(day: u32) -> IoResult<Vec<String>> {
    let path_name = format!("input/{}/input.txt", day);
    let path = Path::new(path_name.as_str());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}