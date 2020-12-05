use crate::input;

struct Grid {
    grid: Vec<Vec<char>>
}

impl Grid {
    fn new(input: &Vec<String>) -> Grid {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in input {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }
        return Grid {
            grid: grid
        }
    }

    fn size_x(&self) -> usize {
        return self.grid.len();
    }

    fn index(&self, x: usize, y: usize) -> char {
        let y_size_at_x = self.grid[x].len();
        let y_wrapped = y % y_size_at_x;
        return self.grid[x][y_wrapped];
    }
}

fn new_movement(step_x: usize, step_y: usize) -> Box<dyn Fn(usize, usize) -> (usize, usize)> {
    return Box::new(move |x, y| (x+step_x, y + step_y))
} 


pub fn question1() -> String {
    let filename = "input/day3.txt";
    let input = input::lines_as::<String>(filename);
    let slope = Grid::new(&input);
    let mut trees = 0;

    for x in 1..slope.size_x() {
        let y = x * 3;
        if slope.index(x, y) == '#' {
            trees += 1;
        }
    }
    return format!("Day2.1: Number of Trees = {}", trees);
}

pub fn question2() -> String {
    let filename = "input/day3.txt";
    let input = input::lines_as::<String>(filename);
    let slope = Grid::new(&input);
    let movements = vec![
        new_movement(1, 1),
        new_movement(1, 3),
        new_movement(1, 5),
        new_movement(1, 7),
        new_movement(2, 1)
        ];

    let mut results = Vec::new();
    for movement in movements {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while x < slope.size_x() {
            if slope.index(x, y) == '#' {
                trees += 1;
            }
            let (a, b) = movement(x, y);
            x = a;
            y = b;
        }
        results.push(trees);
    }

    let multiply = results.iter().fold(1u64, |acc, result| acc * result);
    return format!("Day2.2: Number of Trees = {}", multiply);
}