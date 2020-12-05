use std::collections::HashMap;
use crate::input;
use regex::Regex;


#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>
}

struct PassportField {
    name: String,
    validation: fn(&String)-> bool
}

impl Passport {
    fn read(lines: &Vec<String>, start_index: usize) -> (Passport, usize) 
    {
        let mut passport = Passport {fields: HashMap::new()};

        let mut index = start_index;
        while index < lines.len() && lines[index] != "" {
            let line = &lines[index];
            let kv_pairs: Vec<&str> = line.split(" ").collect();

            for pair in kv_pairs {
                let kv: Vec<&str> = pair.split(":").collect();
                let name = kv[0].to_string();
                let value = kv[1].to_string();
                passport.fields.insert(name, value);
            }
            index += 1;
        }

        return (passport, index + 1);
    }

    fn is_valid(&self) -> bool {
        let fields = vec!["byr","iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for field in fields {
            if !self.fields.contains_key(field) {
                return false;
            }
        }
        return true;
    }

    fn is_proper_valid(&self) -> bool {
        let fields = vec![
            PassportField{
                name: String::from("byr"),
                validation: |value| {
                    let v = value.parse::<i32>().unwrap();
                    return v >= 1920 && v <= 2002;
                }
            },
            PassportField {
                name: String::from("iyr"),
                validation: |value| {
                    let v = value.parse::<i32>().unwrap();
                    return v >= 2010 && v <= 2020;
                }
            },
            PassportField {
                name: String::from("eyr"),
                validation: |value| {
                    let v = value.parse::<i32>().unwrap();
                    return v >= 2020 && v <= 2030;
                }
            },
            PassportField {
                name: String::from("hgt"),
                validation: |value| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new("^([0-9]+)(cm|in)$").unwrap();
                    }
                    match RE.captures(value) {
                        None => return false,
                        Some(capture) => {
                            let v = capture[1].parse::<i32>().unwrap();
                            if capture[2] == String::from("cm") {
                                return v >= 150 && v <= 193;
                            } else if capture[2] == String::from("in") {
                                return v >= 59 && v <= 76;
                            } else {
                                return false;
                            }
                        }
                    }
                }
            },
            PassportField {
                name: String::from("hcl"),
                validation: |value| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
                    }
                    return RE.is_match(value);
                }
            },
            PassportField {
                name: String::from("ecl"),
                validation: |value| {
                    lazy_static! {
                        static ref COLOR: Vec<String> = vec![
                            String::from("amb"),
                            String::from("blu"),
                            String::from("brn"),
                            String::from("gry"),
                            String::from("grn"),
                            String::from("hzl"),
                            String::from("oth")
                        ];
                    }
                    return COLOR.contains(value);
                }
            },
            PassportField {
                name: String::from("pid"),
                validation: |value| {
                    lazy_static! {
                        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
                    }
                    return RE.is_match(value);
                }
            },
        ];

        for field in fields {
            if !self.fields.contains_key(&field.name) {
                return false;
            }
            let value = &self.fields[&field.name];
            let validator = field.validation;
            if validator(&value) == false {
                return false
            }
        }
        return true;
    }
}

pub fn question1() -> String {
    let filename = "input/day4.txt";
    let input = input::lines_as::<String>(filename);
    let mut passports = Vec::new();

    let mut index = 0;
    while index < input.len() {
        let (passport, new_index) = Passport::read(&input, index);
        index = new_index;
        passports.push(passport);
    }

    let mut valid_passports = 0;
    for p in passports {
        if p.is_valid() {
            valid_passports += 1;
        }
    }
    return format!("Day 4.1: valid passports = {}", valid_passports);
}


pub fn question2() -> String {
    let filename = "input/day4.txt";
    let input = input::lines_as::<String>(filename);
    let mut passports = Vec::new();

    let mut index = 0;
    while index < input.len() {
        let (passport, new_index) = Passport::read(&input, index);
        index = new_index;
        passports.push(passport);
    }

    let mut valid_passports = 0;
    for p in passports {
        if p.is_proper_valid() {
            valid_passports += 1;
        }
    }
    return format!("Day 4.1: valid passports = {}", valid_passports);
}