use crate::input;
use std::collections::{HashSet, HashMap};

pub fn question1() -> String {
    let filename = "input/day6.txt";
    let input = input::collect_groups_as::<String>(filename);

    let mut sum = 0;
    for group in input {
        let answers: HashSet<char> = group.join("").chars().collect();
        sum += answers.len();
    }

    return format!("Day 6.1: sum of answers = {}", sum);
}


pub fn question2() -> String {
    let filename = "input/day6.txt";
    let input = input::collect_groups_as::<String>(filename);

    let mut sum = 0;
    for group in input {
        let mut answers: HashMap<char, usize> = HashMap::new();

        for person in &group {
            for answer in person.chars() {
                *answers.entry(answer).or_insert(0) += 1;
            }
        }

        let number_of_people_in_group = group.len();
        let answered_by_all: u32 = answers.iter().map(|(_, v)| {
            if *v == number_of_people_in_group {
                return 1;
            } else {
                return 0;
            }
        }).sum();

        sum += answered_by_all;
    }

    return format!("Day 6.2: sum of answers = {}", sum);
}