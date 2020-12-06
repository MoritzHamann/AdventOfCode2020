use crate::input;

fn find_entry(iter: &mut std::str::Chars, lower: u32, upper: u32, down: char, up: char) -> u32 {
    let mut min = lower;
    let mut max = upper;

    while min != max {
        let p = iter.next().unwrap();
        if p == down {
            max = (max - min) / 2 + min;
        } else if p == up {
            min = (max - min) / 2 + 1 + min;

        } else {
            panic!("invalid char")
        }
    }
    return min;

}

fn get_row(iter: &mut std::str::Chars) -> u32 {
    return find_entry(iter, 0, 127, 'F', 'B');
}

fn get_column(iter: &mut std::str::Chars) -> u32 {
    return find_entry(iter, 0, 7, 'L', 'R');
}

fn get_id(partition: &String) -> u32 {
    let mut iter = partition.chars();
    let row = get_row(&mut iter);
    let column = get_column(&mut iter);
    return row * 8 + column;
}

pub fn question1() -> String {
    let filename = "input/day5.txt";
    let input = input::lines_as::<String>(filename);
    let max = input.iter().map(get_id).max().unwrap();
    return format!("Day 5.1: maxium id = {}", max);
}

pub fn question2() -> String {
    let filename = "input/day5.txt";
    let input = input::lines_as::<String>(filename);
    let mut ids = input.iter().map(get_id).collect::<Vec<u32>>();
    ids.sort();

    for i in 1..ids.len()-1 {
        let prev = ids[i-1];
        let current = ids[i];

        if current == prev + 2 {
            return format!("Day 5.2: my id = {}", prev + 1);
        }
    }

    return format!("{}", "not found");
}