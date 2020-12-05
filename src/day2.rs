
use crate::input;
use regex::Regex;

fn is_valid_password(entry: &PasswordEntry) -> bool {
    let mut occurrence = 0;
    for c in entry.pw.chars() {
        if c == entry.c {
            occurrence += 1;
        }
    }

    return occurrence >= entry.min && occurrence <= entry.max;
}

fn is_valid_password_correct_policy(entry: &PasswordEntry) -> bool {
    let first_letter = entry.pw.chars().nth(entry.min as usize).unwrap() == entry.c;
    let second_letter = entry.pw.chars().nth(entry.max as usize).unwrap() == entry.c;

    return (first_letter || second_letter) && !(first_letter && second_letter);
}

struct PasswordEntry {
    min: i32,
    max: i32,
    c: char,
    pw: String,
}

impl PasswordEntry {
    fn from_line(line: &String) -> PasswordEntry {
        lazy_static! {
            static ref RE: Regex = Regex::new("([0-9]+)-([0-9]+) (.):(.*)").unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        let min = captures[1].parse::<i32>().unwrap();
        let max = captures[2].parse::<i32>().unwrap();
        let character = captures[3].parse::<char>().unwrap();
        let pw = &captures[4];
        return PasswordEntry {
            min: min,
            max: max,
            c: character,
            pw: pw.to_string()
        }
    }
}

pub fn question1() -> String {
    let filename = "input/day2.txt";
    let input = input::lines_as::<String>(filename);

    let mut valid_passwords = 0;
    for line in input {
        let entry = PasswordEntry::from_line(&line);

        if is_valid_password(&entry) {
            valid_passwords += 1;
        }
    }

    return format!("Day 2.1: valid passwords = {}", valid_passwords);
}

pub fn question2() -> String {
    let filename = "input/day2.txt";
    let input = input::lines_as::<String>(filename);

    let mut valid_passwords = 0;
    for line in input {
        let entry = PasswordEntry::from_line(&line);

        if is_valid_password_correct_policy(&entry) {
            valid_passwords += 1;
        }
    }

    return format!("Day 2.2: valid passwords = {}", valid_passwords);
}