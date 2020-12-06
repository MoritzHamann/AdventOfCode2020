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

pub fn collect_groups_as<T>(filename: &str) -> Vec<Vec<T>>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
    {
    let mut groups = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut current_group = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();

        if l == String::from("") {
            groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(l.parse::<T>().unwrap());
        }
    }
    groups.push(current_group);
    
    return groups;
}