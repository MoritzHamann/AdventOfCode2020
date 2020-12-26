use crate::input;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
enum Space {
    Floor,
    Empty,
    Occupied
}
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::Floor => write!(f, "."),
            Space::Empty => write!(f, "L"),
            Space::Occupied => write!(f, "#")
        }
    }
}

struct Grid {
    grid: Vec<Vec<Space>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Grid {
    fn new(input: &Vec<String>) -> Grid {
        let mut grid = Vec::new();
        for line in input {
            let mut row: Vec<Space> = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '.' => Space::Floor,
                    'L' => Space::Empty,
                    _ => panic!("unkown input {}", c)
                })
            }
            grid.push(row);
        }
        
        return Grid {
            grid: grid,
        };
    }

    fn num_rows(&self) -> usize {
        return self.grid.len();
    }

    fn num_cols(&self) -> usize {
        return self.grid.first().map(|r| r.len()).unwrap_or(0);
    }

    fn at(&self, row: i32, col: i32) -> Option<&Space> {
        if row < 0 || row < 0 {
            return None;
        }

        let row = row as usize;
        let col = col as usize;
        if  row < self.num_rows() && col < self.num_cols() {
            return Some(&self.grid[row][col]);
        }
        return None;
    }


    // returns if the grid changed during the step
    fn next_step(
        &mut self,
        change_policy: &dyn Fn(&Space, u8) -> Space,
        neighbor_policy: &dyn Fn(&Grid, i32, i32) -> u8) -> bool 
    {
        let mut new_grid:Vec<Vec<Space>> = Vec::new();
        let mut changed = false;

        for row in 0..self.num_rows() {
            let mut new_row: Vec<Space> = Vec::new();

            for col in 0..self.num_cols() {
                let num_occupied_neighbors = neighbor_policy(&self, row as i32, col as i32);
                let value = self.at(row as i32 , col as i32);
                let new_value = match value {
                    Some(value) => change_policy(value, num_occupied_neighbors),
                    None => panic!("out of bound: {}, {}", row, col)
                };

                // short circut the changed value, to ensure we don't need to compare old
                // and new grid after step
                if value != Some(&new_value) {
                    changed = true;
                }
                new_row.push(new_value);
            }
            new_grid.push(new_row);
        }

        // set new grid
        self.grid = new_grid;
        return changed;
    }

    fn num_occupied_seats(&self) -> u32 {
        let mut seats = 0;
        for row in &self.grid {
            for col in row {
                if col == &Space::Occupied {
                    seats +=1;
                }
            }
        }
        return seats;
    }
}

fn change_policy(space: &Space, num_occupied_neighbors: u8) -> Space {
    match space {
        Space::Occupied if num_occupied_neighbors >= 4 => Space::Empty,
        Space::Empty if num_occupied_neighbors == 0 => Space::Occupied,
        value => value.clone(),
    }
}

fn change_policy2(space: &Space, num_occupied_neighbors: u8) -> Space {
    match space {
        Space::Occupied if num_occupied_neighbors >= 5 => Space::Empty,
        Space::Empty if num_occupied_neighbors == 0 => Space::Occupied,
        value => value.clone(),
    }
}

fn num_occupied_direct_neighbors(grid: &Grid, row: i32, col: i32) -> u8 {
    let neighbors = vec![
        (row-1, col),
        (row-1, col+1),
        (row,   col+1),
        (row+1, col+1),
        (row+1, col),
        (row+1, col-1),
        (row,   col-1),
        (row-1, col-1)];
    let mut occupied_neighbors = 0;

    for (r, c) in neighbors {
        let value = grid.at(r, c);
        match value {
            Some(Space::Occupied) => occupied_neighbors += 1,
            _ => ()
        }
    }

    return occupied_neighbors;
}

fn num_occupied_visible_neighbors(grid: &Grid, row: i32, col: i32) -> u8 {
    let directions = vec![
        (-1,  0),
        (-1,  1),
        (0,   1),
        (1,   1),
        (1,   0),
        (1,  -1),
        (0,  -1),
        (-1, -1)];

    let mut occupied_neighbors = 0;
    for (dx, dy) in directions {
        let mut x = row + dx;
        let mut y = col + dy;

        while grid.at(x, y).map_or(false, |p| p == &Space::Floor) {
            x += dx;
            y += dy;
        }
        if grid.at(x, y).map_or(false, |p| p == &Space::Occupied) {
            occupied_neighbors += 1;
        }
    }
    return occupied_neighbors;
}

pub fn question1() -> String {
    let filename = "input/day11.txt";
    let input = input::lines_as::<String>(filename);
    let mut grid = Grid::new(&input);

    while grid.next_step(&change_policy, &num_occupied_direct_neighbors) {
    }
    let occupied_seats = grid.num_occupied_seats();

    return format!("Day 11.1: occupied seats = {}", occupied_seats);
}

pub fn question2() -> String {
    let filename = "input/day11.txt";
    let input = input::lines_as::<String>(filename);
    let mut grid = Grid::new(&input);

    while grid.next_step(&change_policy2, &num_occupied_visible_neighbors) {
    }

    let occupied_seats = grid.num_occupied_seats();
    return format!("Day 11.2: occupied seats = {}", occupied_seats);
}