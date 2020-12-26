use crate::input;

#[derive(Debug)]
enum Direction {North, East, South, West}

#[derive(Debug)]
enum Action {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward
}
impl Action {
    fn from_string(str: &char) -> Action {
        return match str {
            'N' => Action::North,
            'E' => Action::East,
            'S' => Action::South,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            _ => panic!("Unknown action {}", str)
        };
    }
}


#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32
}
impl Instruction {
    fn from_string(line: &String) -> Instruction {
        let action = Action::from_string(&line.chars().nth(0).unwrap());
        let value = String::from(&line[1..]).parse::<i32>().unwrap();
        Instruction {
            action: action,
            value: value
        }
    }
}


#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32
}
impl Waypoint {
    fn new(x: i32, y: i32) -> Waypoint {
        return Waypoint {
            x: x,
            y: y
        }
    }

    fn rotate(&mut self, degree: i32) {
        let degree_f = (-degree as f64).to_radians();
        let x = self.x as f64;
        let y = self.y as f64;

        // we'll just assume the int conversion will always work :D
        unsafe {
            let new_x = x * degree_f.cos() - y * degree_f.sin();
            let new_y = x * degree_f.sin() + y * degree_f.cos();
            self.x = new_x.round().to_int_unchecked();
            self.y = new_y.round().to_int_unchecked();
        }
    }
    fn move_rel(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy
    }
}

#[derive(Debug)]
struct Ferry {
    direction: i32,
    x: i32,
    y: i32,
    waypoint: Waypoint
}
impl Ferry {
    fn new(direction: i32, x: i32, y:i32) -> Ferry {
        return Ferry {
            direction: direction,
            x: x,
            y: y,
            waypoint: Waypoint::new(0, 0)
        };
    }

    fn new_with_waypoint(direction: i32, x: i32, y:i32, waypoint: Waypoint) -> Ferry {
        return Ferry {
            direction: direction,
            x: x,
            y: y,
            waypoint: waypoint
        };

    }

    // Coordinate system:
    // (y)
    // ^
    // |
    // |
    // -----> (x)
    //
    // North == 0
    // East == 90
    fn follow_instruction(&mut self, instruction: &Instruction) {
        let amount = instruction.value;
        match instruction.action {
            Action::North => self.move_rel(0, amount),
            Action::East => self.move_rel(amount, 0),
            Action::South => self.move_rel(0, -amount),
            Action::West => self.move_rel(-amount, 0),
            Action::Left => self.direction -= amount,
            Action::Right => self.direction += amount,
            Action::Forward => {
                // to lazy to use trigonometry and conversion to/from float
                let dir = self.direction % 360;
                if dir == 0 {
                    self.move_rel(0, amount);
                } else if dir == 90 || dir == -270 {
                    self.move_rel(amount, 0);
                } else if dir == 180 || dir == -180 {
                    self.move_rel(0, -amount);
                } else if dir == 270 || dir == -90 {
                    self.move_rel(-amount, 0);
                }
            }
        }
    }

    fn move_rel(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn follow_instruction_with_waypoint(&mut self, instruction: &Instruction) {
        let amount = instruction.value;
        match instruction.action {
            Action::North => self.waypoint.move_rel(0, amount),
            Action::East => self.waypoint.move_rel(amount, 0),
            Action::South => self.waypoint.move_rel(0, -amount),
            Action::West => self.waypoint.move_rel(-amount, 0),
            Action::Left => self.waypoint.rotate(-amount),
            Action::Right => self.waypoint.rotate(amount),
            Action::Forward => {
                let dx = self.waypoint.x * amount;
                let dy = self.waypoint.y * amount;
                self.move_rel(dx, dy);
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}


pub fn question1() -> String {
    let filename = "input/day12.txt";
    let instructions: Vec<Instruction> =
        input::lines_as::<String>(filename).iter().map(Instruction::from_string).collect();
    let mut ferry = Ferry::new(90, 0, 0);

    for instruction in instructions {
        ferry.follow_instruction(&instruction);
    }
    
    return format!("Day 12.1: distance from origin = {}", ferry.manhattan_distance());
}

pub fn question2() -> String {
    let filename = "input/day12.txt";
    let instructions: Vec<Instruction> =
        input::lines_as::<String>(filename).iter().map(Instruction::from_string).collect();
    let waypoint = Waypoint::new(10, 1);
    let mut ferry = Ferry::new_with_waypoint(0, 0, 0, waypoint);

    for instruction in &instructions {
        ferry.follow_instruction_with_waypoint(instruction);
    }

    return format!("Day 12.2: distance from origin = {}", ferry.manhattan_distance());
}