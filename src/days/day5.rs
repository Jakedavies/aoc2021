use regex::Regex;
use std::cmp;
use std::collections::HashMap;


#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}



fn parse_input(input: &str) -> Vent {
    let parser: Regex = Regex::new(r"^(\d*),(\d*) -> (\d*),(\d*)$").unwrap();

    let captures = parser.captures(input).unwrap();
    let x1 = captures.get(1).unwrap().as_str();
    let y1 = captures.get(2).unwrap().as_str();
    let x2 = captures.get(3).unwrap().as_str();
    let y2 = captures.get(4).unwrap().as_str();

    Vent {
        x1: i32::from_str_radix(x1, 10).unwrap(),
        y1: i32::from_str_radix(y1, 10).unwrap(),
        x2: i32::from_str_radix(x2, 10).unwrap(),
        y2: i32::from_str_radix(y2, 10).unwrap()
    }
}

fn coord_to_key(x: i32, y: i32) -> String {
    format!("{},{}", x, y)
}

fn add_to_pointmap(map: &mut HashMap<String, u32>, x: i32, y: i32) {
    let key = coord_to_key(x, y);
    if map.contains_key(&key) {
        let v = *map.get_mut(&key).unwrap();
        map.insert(key, v + 1);
    } else {
        map.insert(key, 1);
    }
} 

pub fn part1() {
    let vents: Vec<Vent> = include_str!("./../../inputs/input5")
        .lines()
        .map(|f| parse_input(f))
        .collect();

    let mut points: HashMap<String, u32> = HashMap::new();

    for vent in vents {
        if vent.x1 == vent.x2 {
            for y in cmp::min(vent.y1, vent.y2)..cmp::max(vent.y2+1, vent.y1+1) {
                add_to_pointmap(&mut points, vent.x1, y)
            }
        }

        if vent.y1 == vent.y2 {
            for x in cmp::min(vent.x1, vent.x2)..cmp::max(vent.x2+1, vent.x1+1) {
                add_to_pointmap(&mut points, x, vent.y1)
            }
        }
    } 

    let mut count = 0;

    for val in points.values() {
        if *val > 1 {
            count += 1;
        }
    }
    println!("{} points overlap", count);
}

pub fn part2() {
    let vents: Vec<Vent> = include_str!("./../../inputs/input5")
        .lines()
        .map(|f| parse_input(f))
        .collect();

    let mut points: HashMap<String, u32> = HashMap::new();

    for vent in vents {
        if vent.x1 == vent.x2 {
            for y in cmp::min(vent.y1, vent.y2)..cmp::max(vent.y2+1, vent.y1+1) {
                add_to_pointmap(&mut points, vent.x1, y)
            }
        } else if vent.y1 == vent.y2 {
            for x in cmp::min(vent.x1, vent.x2)..cmp::max(vent.x2+1, vent.x1+1) {
                add_to_pointmap(&mut points, x, vent.y1)
            }
        } else {
            let mut x = vent.x1;
            let mut y = vent.y1;
            let x_step = if vent.x1 < vent.x2 { 1 } else { -1 }; 
            let y_step = if vent.y1 < vent.y2 { 1 } else { -1 }; 
            while y != vent.y2 + y_step {
                add_to_pointmap(&mut points, x, y);
                y += y_step;    
                x += x_step;    
            }
        }
    } 

    let mut count = 0;

    for val in points.values() {
        if *val > 1 {
            count += 1;
        }
    }
    println!("{} points overlap", count);
}
