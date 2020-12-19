use crate::input;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

struct RingBuffer {
    // current index into buffer
    index: usize,
    size: usize,

    // buffer of elements
    buffer: Vec<u64>,

    // sorted map which counts the elements in the buffer vector
    // since BTreeMap is sorted on keys, it allows to stop the 
    // iteration once a + b > treshold (since all subsequent b will be
    // bigger anyways)
    elements: BTreeMap<u64, u8>
}

impl RingBuffer {
    fn fill(input: &Vec<u64>, size: usize) -> RingBuffer {
        let mut buffer = RingBuffer {
            index: 0,
            size: size,
            buffer: Vec::new(),
            elements: BTreeMap::new()
        };


        for idx in 0..size {
            let value = input[idx];
            buffer.buffer.push(value); 
            *buffer.elements.entry(value).or_insert(0) += 1;
        }

        return buffer;
    }

    fn is_valid_next_number(&self, value: u64) -> bool {
        // somewhat optmizied approach.
        // we rely on the fact that the btree map has sorted keys.
        // the outer iteration defines the lower bound of our sum,
        // while the inner iteration adds the remaining elements to the lower bound.
        // if the sum is to high, this index is the new upper bound, since any the values
        // of the lower bound will only increase.
        //
        // Example for target value of 17
        // step 1:
        // [ 1, 3, 5, 7, 12, 15, 20, 35]
        //   |                        |
        // lower bound                upper bound
        //
        // step 2:
        // [ 1, 3, 5, 7, 12, 15, 20, 35]
        //      |                 |
        //    lower bound        upper bound
        //
        // step 3:
        // [ 1, 3, 5, 7, 12, 15, 20, 35]
        //         |          |
        //    lower bound     upper bound

        let mut upper_bound_idx = self.elements.len();

        for lower_bound_idx in 0..self.elements.len() {
            // using .nth() is suboptiomal, since it takes liner time to access it.
            // potential room for optimizations by keeping iterators around
            let (lower_val, _) = self.elements.iter().nth(lower_bound_idx).unwrap();

            for idx in lower_bound_idx..upper_bound_idx {
                let (current_val, _) = self.elements.iter().nth(idx).unwrap(); 
                let sum = lower_val + current_val;
                if sum == value {
                    return true;
                } else if sum > value {
                    upper_bound_idx = idx;
                    break;
                }
            }
        }
        return false
    }

    fn insert_next_value(&mut self, value: u64) -> bool {
        if !self.is_valid_next_number(value) {
            return false;
        }

        // update the ring buffer
        let old_value = self.buffer[self.index];
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % self.size;

        // update old entry in the elements map
        let entry = self.elements.entry(old_value);
        match entry  {
            Entry::Vacant(_) => panic!("no entry for old value present"),
            Entry::Occupied(mut e) => {
                if e.get() == &1 {
                    e.remove();
                } else {
                    e.insert(e.get() - 1);
                }
            }
        }

        // insert the new entry in the elements map
        self.elements.insert(value, 1);

        return true;
    }
}

pub fn question1() -> String {
    let filename = "input/day9.txt";
    let input = input::lines_as::<u64>(filename);
    let amount = 25;
    let mut ring_buffer = RingBuffer::fill(&input, amount);

    for idx in amount..input.len() {
        let val = input[idx];
        let is_valid = ring_buffer.insert_next_value(val);
        if !is_valid {
            return format!("Day 9.1: first wrong number = {}", val);
        }
    }

    return format!("Day 9.1: All values are valid");
}


pub fn question2() -> String {
    let invalid_number = 373803594;
    let filename = "input/day9.txt";
    let input = input::lines_as::<u64>(filename);

    let mut lower_bound_idx = 0;
    let mut upper_bound_idx = 0;
    let mut sum = 0;

    while upper_bound_idx < input.len() {
        sum = input[lower_bound_idx..upper_bound_idx+1].iter().sum();

        if sum == invalid_number {
            let min = input[lower_bound_idx..upper_bound_idx+1].iter().min().unwrap();
            let max = input[lower_bound_idx..upper_bound_idx+1].iter().max().unwrap();
            return format!("Day 9.2: sum of min and max of range = {}", min + max)
        } else if sum < invalid_number{
            upper_bound_idx += 1;
        } else if sum > invalid_number {
            lower_bound_idx += 1;
        }
    }

    return format!("Day 9.2: no range found");
}