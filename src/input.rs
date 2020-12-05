use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lines_as<T>(filename: &str) -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input = Vec::new();

    for line in reader.lines() {
        let value = line.unwrap().parse::<T>().unwrap();
        input.push(value);
    }
    return input;
}