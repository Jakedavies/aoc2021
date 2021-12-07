use std::ops::Add;
use crate::utils::file_utils;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Forward,
    Back
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);


impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Aim(i32);

impl Add for Aim {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}


fn parse_command(instruction: &str) -> Instruction {
    let parts: Vec<&str> = instruction.split(" ").collect();

    let direction = match parts[0] {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "forward" => Direction::Forward,
        "back" => Direction::Back,
        _ => panic!("Unrecognized direction {}", parts[0])
    };

    if let Ok(amount) = i32::from_str_radix(parts[1],10) { 
        Instruction {
            direction,
            amount
        }
    } else {
        panic!("Could not parse amount: {}", parts[1])
    }
}


fn apply_command(origin: Point, command: &str) -> Point {
    let instruction = parse_command(&command);

    let movement_vector = match instruction.direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Forward => (1, 0),
        Direction::Back => (-1, 0),
    };

    origin + Point(movement_vector.0 * instruction.amount, movement_vector.1 * instruction.amount)
}

fn apply_command_part2(aim: Aim, origin: Point, command: &str) -> (Aim, Point) {
    let instruction = parse_command(&command);

    match instruction.direction {
        Direction::Up => (aim + Aim(-instruction.amount), origin),
        Direction::Down => (aim + Aim(instruction.amount), origin),
        Direction::Forward => {
            let old_aim = Aim(aim.0);
            (aim, origin + Point(instruction.amount, old_aim.0 * instruction.amount))
        },
        direction => panic!("{:?} is invalid", direction)
    }
}

pub fn part1() {
    let mut origin = Point(0, 0);

    if let Ok(lines) = file_utils::read_lines("./inputs/input2") {
        for line in lines {
            if let Ok(instruction_string) = line {
                origin = apply_command(origin, &instruction_string)
            }
        }
    }

    println!("{}, {}", origin.0, origin.1)
}

pub fn part2() {
    let mut origin = Point(0, 0);
    let mut aim = Aim(0);

    if let Ok(lines) = file_utils::read_lines("./inputs/input2") {
        for line in lines {
            if let Ok(instruction_string) = line {
                let result = apply_command_part2(aim, origin, &instruction_string);
                aim = result.0;
                origin = result.1;
            }
        }
    }

    println!("{}, {}", origin.0, origin.1)
}


#[cfg(test)]
mod tests {
    #[test]
    fn direction_parse_up() {
        assert_eq!(super::parse_command("up 5"), super::Instruction{ direction: super::Direction::Up, amount: 5 })
    }

    #[test]
    fn test_case_1() {
        let mut pos = super::Point(0, 0);
        pos = super::apply_command(pos, "forward 5");
        pos = super::apply_command(pos, "down 5");
        pos = super::apply_command(pos, "forward 8");
        pos = super::apply_command(pos, "up 3");
        pos = super::apply_command(pos, "down 8");
        pos = super::apply_command(pos, "forward 2");
        assert_eq!(pos, super::Point(15, 10));
    }

    #[test]
    fn test_case_2() {
        let pos = super::Point(0, 0);
        let aim = super::Aim(2);
        let (new_aim, new_pos) = super::apply_command_part2(aim, pos, "forward 5");
        assert_eq!(new_pos, super::Point(5, 10));
        assert_eq!(new_aim, super::Aim(2));
    }
}
