use crate::utils::file_utils;


pub fn part1() {
    let mut incremented_count = 0;
    // lol
    let mut last_num: i32 = 2147483647;
    if let Ok(lines) = file_utils::read_lines("./inputs/input1") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(string_num) = line {
                if let Ok(num) = i32::from_str_radix(&string_num, 10) {
                    if num > last_num {
                        incremented_count += 1;
                    }
                    last_num = num;
                }
            }
        }
    }
    println!("Number increased {} times", incremented_count)
}

pub fn part2() {
    let mut incremented_count = 0;
    // lol
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(lines) = file_utils::read_lines("./inputs/input1") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(string_num) = line {
                if let Ok(num) = i32::from_str_radix(&string_num, 10) {
                    println!("{}", num);
                    numbers.push(num);
                }
            }
        }
    }
    for i in 0..numbers.len()-3 {
        println!("Window start {}, end {}", &numbers[i], &numbers[i+3]);
        if &numbers[i..(i+3)].iter().sum::<i32>() < &numbers[i+1..i+4].iter().sum::<i32>() {
            incremented_count += 1;
        }
    }
    println!("Number increased {} times", incremented_count)
}
