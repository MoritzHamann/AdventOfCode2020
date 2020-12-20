use crate::input;
use std::collections::HashMap;


pub fn question1() -> String {
    let filename = "input/day10.txt";
    let mut input = input::lines_as::<u32>(filename);
    let mut jolts: HashMap<u32, u32> = HashMap::new();

    input.sort();
   
    let mut prev = 0;
    for plug in input {
        let diff = plug - prev;
        prev = plug;
        *jolts.entry(diff).or_insert(0) += 1;
    }

    // add a 3 jolt diff for the last adaptor to the device
    *jolts.entry(3).or_insert(0) += 1;

    let solution = jolts[&1] * jolts[&3];
    return format!("Day 10.1: {} (1 jolt) * {} (3 jolt) = {}", jolts[&1], jolts[&3], solution);
}

pub fn prev_neighbors(idx: usize, input: &Vec<u32>) -> &[u32] {
    let val = input[idx];
    let mut lower_bound = idx;
    while lower_bound > 0 && (val - input[lower_bound-1]) <=3 {
        lower_bound -= 1;
    }
    return &input[lower_bound..idx];
}

// using som form of dynamic programming:
// in the sequence of adapters
// 0|1,2,3,4,7|10
// 1 can be reached by 0, so number of possibilites = 1
// 2 can be reached by 0 and 1, so pos(2) = pos(1) + 1 = 2
// 3 can be reached by 0, 1 and 2, so pos(3) = pos(2) + pos(1) + 1 = 2 + 1 + 1 = 4
// 4 can be reached by 1, 2 and 3, so pos(4) = pos(3) + pos(2) + pos(1) = 4 + 2 + 1 = 7
// 7 can be reached by 4, so pos(7) = pos(4) = 7
// 10 can be reached by 7, so pos(10) = pos(7) = 7
// => 7 total possibilites
pub fn question2() -> String {
    let filename = "input/day10.txt";
    let mut input = input::lines_as::<u32>(filename);
    input.sort();
    input.insert(0, 0);

    let mut combinations: HashMap<u32, u64> = HashMap::new();
    combinations.insert(0, 1);

    for idx in 1..input.len() {
        let adapter = input[idx];
        let neighbors = prev_neighbors(idx, &input);
        let sum = neighbors.iter().map(|i| combinations[i]).sum();
        combinations.insert(adapter, sum);
    }

    let num_combinations = combinations[input.last().unwrap()];
    return format!("Day 10.2: number of ways to arrange adapters = {}", num_combinations);
}
