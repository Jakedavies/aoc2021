use std::cmp;
use std::collections::HashMap;

pub fn part1() {
    let lines: Vec<&str> = include_str!("./../../inputs/input7")
        .lines()
        .collect();

    let positions: Vec<i64> = lines[0]
        .split(',')
        .map(|f| i64::from_str_radix(f, 10).unwrap())
        .collect();

    let max = positions.iter().max().unwrap();

    let mut min_cost = 100000000;
    for i in 0..*max {
        let mut cost: i64 = 0;
        for position in &positions {
            cost += (i - position).abs();
        }
        min_cost = cmp::min(cost, min_cost)
    }
    print!("min cost {}", min_cost)
}


pub fn part2() {
    let lines: Vec<&str> = include_str!("./../../inputs/input7")
        .lines()
        .collect();

    let positions: Vec<i64> = lines[0]
        .split(',')
        .map(|f| i64::from_str_radix(f, 10).unwrap())
        .collect();

    let max = positions.iter().max().unwrap();

    let mut distance_to_fuel_cost = HashMap::<i64, i64>::new();
    distance_to_fuel_cost.insert(0, 0);
    for i in 1..(*max+1) {
        let previous_cost = *distance_to_fuel_cost.get(&(i - 1)).unwrap();
        distance_to_fuel_cost.insert(i, previous_cost + i);
    }

    let mut min_cost = 100000000;
    for i in 0..*max {
        let mut cost: i64 = 0;
        for position in &positions {
            let move_distance = (i - position).abs();
            cost += distance_to_fuel_cost.get(&move_distance).unwrap();
        }
        min_cost = cmp::min(cost, min_cost)
    }
    print!("min cost {}", min_cost)
}

