use crate::input;
use std::collections::{HashMap, HashSet};
use regex::Regex;

type Bag = String;

struct BagRule {
    color: String,
    contains: Vec<(u32, Bag)>
}

impl BagRule {
    fn from_string(description: &String) -> BagRule {
        lazy_static! {
            static ref RE_BAG_COLOR: Regex = Regex::new(
                r"([a-zA-Z]+ [a-zA-Z]+) bags contain").unwrap();
            static ref RE_BAG_CONTAINS: Regex = Regex::new(
                r"(?:(\d+) ([a-zA-Z]+ [a-zA-Z]+) bags?[.|,])").unwrap();
        }
        let color = String::from(&RE_BAG_COLOR.captures(description).unwrap()[1]);
        let contains: Vec<(u32, String)> =
            RE_BAG_CONTAINS.captures_iter(description)
                .map(|capture|
                    (String::from(&capture[1]).parse::<u32>().unwrap(), String::from(&capture[2]))
                )
                .collect();

        return BagRule {
            color: color,
            contains: contains
        }
    }
}

struct BagRules {
    is_contained_in: HashMap<Bag, Vec<(u32, Bag)>>,
    rules: HashMap<Bag, BagRule>
}

impl BagRules {
    fn new() -> BagRules {
        BagRules {
            is_contained_in: HashMap::new(),
            rules: HashMap::new()
        }
    }

    fn add_rule(&mut self, bag_rule: BagRule) {
        self.rules.insert(bag_rule.color.clone(), bag_rule);
        ()
    }

    fn add_inverse_rule(&mut self, bag_rule: BagRule) {
        let color = bag_rule.color;
        for (number, enclosing_color) in bag_rule.contains {
            let list_of_enclosing_colors = self.is_contained_in.entry(enclosing_color).or_insert(Vec::new());
            list_of_enclosing_colors.push((number, color.clone()));
        }
    }

    fn collect_all_enclosing_bags(&self, bag: &Bag) -> HashSet<Bag> {
        let mut collected_bags = HashSet::new();
        let enclosing_bags = self.is_contained_in.get(bag);

        match enclosing_bags {
            None => (),
            Some(vector) => {
                for (_, color) in vector {
                    collected_bags.insert(color.clone());
                    collected_bags.extend(self.collect_all_enclosing_bags(color));
                }
            }
        }
        return collected_bags;
    }

    fn number_of_required_bags(&self, bag: &Bag) -> u32 {
        let mut required_bags = 0;

        match self.rules.get(bag) {
            None => (),
            Some(rule) => {
                for (number, color) in &rule.contains {
                    required_bags += number + number * self.number_of_required_bags(color);
                }
            }
        }
        return required_bags;
    }
}


pub fn question1() -> String {
    let filename = "input/day7.txt";
    let input = input::lines_as::<String>(filename);

    let mut bag_rules = BagRules::new();
    for rule in &input {
        let bag_rule = BagRule::from_string(rule);
        bag_rules.add_inverse_rule(bag_rule);
    }

    let bags = bag_rules.collect_all_enclosing_bags(&String::from("shiny gold"));
    let number_of_bags = bags.len();

    return format!("Day 7.1: number of bag colors containing a shiny gold bag = {}", number_of_bags);
}

pub fn question2() -> String {
    let filename = "input/day7.txt";
    let input = input::lines_as::<String>(filename);

    let mut bag_rules = BagRules::new();
    for rule in &input {
        let bag_rule = BagRule::from_string(rule);
        bag_rules.add_rule(bag_rule);
    }
    let number_of_individual_bags = bag_rules.number_of_required_bags(&String::from("shiny gold"));
    return format!("Day 7.2: number of individual bags = {}", number_of_individual_bags);
}