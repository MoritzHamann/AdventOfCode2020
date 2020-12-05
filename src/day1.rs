
use crate::input;

fn is2020_by2(a: i32, b: i32) -> bool {
    return a + b == 2020;
}

fn is2020_by3(a: i32, b: i32, c: i32) -> bool {
    return a + b + c == 2020;
}

 
pub fn question1() -> String {
    let input_file = "input/day1.txt";
    let input = input::lines_as::<i32>(input_file);

    let length = input.len();
    for i in 0..length {
        for j in (i+1)..length {
            let a = input[i];
            let b = input[j];
            if is2020_by2(a, b) {
                return format!("Day 1.1: {} * {} = {}", a, b, a*b);
            }
        }
    }
    return "No solution found".to_string();
}

pub fn question2() -> String {
    let input_file = "input/day1.txt";
    let input = input::lines_as::<i32>(input_file);

    let length = input.len();
    for i in 0..length {
        for j in (i+1)..length {
            for k in (j+1)..length {
                let a = input[i];
                let b = input[j];
                let c = input[k];
                if is2020_by3(a, b, c) {
                    return format!("Day 1.2: {} * {} * {} = {}", a, b, c, a*b*c);
                }
            }
        }
    }
    return "No solution found".to_string();
}