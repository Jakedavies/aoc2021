use crate::utils::file_utils;

pub fn process_binary_string(mut counts: Vec<i32>, binary: &str) -> Vec<i32> {
    let mut chars = binary.chars();
    for i in 0..binary.len() {
        if let Some(safe_c) = chars.nth(0) {
            if i >= counts.len() {
                counts.push(0) 
            }

            if safe_c == "1".chars().nth(0).unwrap() {
                counts[i] = counts[i] + 1;
            } else {
                counts[i] = counts[i] - 1;
            }

        }
    }
    counts
}

fn invert_binary_string(s: &str) -> String {
    let mut new_str: String = "".to_owned();

    let mut chars = s.chars();

    while let Some(safe_s) = chars.nth(0) {
        if safe_s == "1".chars().nth(0).unwrap()  {
            new_str += "0";
        } else {
            new_str += "1";
        }
    };

    new_str
}

fn count_to_binary_string(counts: Vec<i32>) -> String {
    let mut result = "".to_owned();
    for count in counts {
        if count > 0 {
            result += "1";
        } else {
            result += "0";
        }
    }

    result
}

pub fn part1() {
    let mut counts = vec![];

    if let Ok(lines) = file_utils::read_lines("./inputs/input3") {
        for line in lines {
            if let Ok(instruction_string) = line {
                counts = process_binary_string(counts, &instruction_string)
            }
        }
    }

    let mut result: String = count_to_binary_string(counts);

    println!("{}", i32::from_str_radix(&*result, 2).unwrap());
    println!("{}", i32::from_str_radix(&invert_binary_string(&*result), 2).unwrap());
}

pub fn extract_oxygen(mut counts: Vec<Vec<i32>>) -> Vec<i32>{
    for i in 0..counts[0].len() {
        if counts.len() <= 1 {
            return counts[0].clone();
        }
        let mut sum = 0;
        for j in 0..counts.len() {
            sum += counts[j][i];
        }
        // filter down to right list
        if sum >= 0 {
           counts = counts.iter().filter(|c| c[i] >= 0 ).cloned().collect(); 
        } else {
           counts = counts.iter().filter(|c| c[i] < 0 ).cloned().collect(); 
        }
    }

    counts[0].clone()
}

pub fn extract_co2(mut counts: Vec<Vec<i32>>) -> Vec<i32> {
    for i in 0..counts[0].len() {
        if counts.len() <= 1 {
            return counts[0].clone();
        }
        let mut sum = 0;
        for j in 0..counts.len() {
            sum += counts[j][i];
        }
        // if less 0 than 1
        if sum >= 0 {
           counts = counts.iter().filter(|c| c[i] < 0 ).cloned().collect(); 
        } else {
           counts = counts.iter().filter(|c| c[i] > 0 ).cloned().collect(); 
        }
    }

    counts[0].clone()
}

pub fn part2() {
    let mut counts: Vec<Vec<i32>> = vec![];
    if let Ok(lines) = file_utils::read_lines("./inputs/input3") {
        for line in lines {
            if let Ok(binary_string) = line {
                let c = process_binary_string(vec![], &binary_string);
                counts.push(c)
            }
        }
    }


    let oxygen = extract_oxygen(counts.clone());

    let result1: String = count_to_binary_string(oxygen.clone());

    let co2 = extract_co2(counts);

    let result2: String = count_to_binary_string(co2.clone());

    println!("Found oxygen reading: {:?}", i32::from_str_radix(&*result1, 2).unwrap());
    println!("Found co2 reading: {:?}", i32::from_str_radix(&*result2, 2).unwrap());
}




#[cfg(test)]
mod tests {
    #[test]
    fn process_character() {
        let mut counts = vec![];
        counts = super::process_binary_string(counts, "010100");

        assert_eq!(counts, vec![-1,1,-1,1,-1,-1])
    }

    #[test]
    fn test_health_sensors() {
        let example_case: Vec<Vec<i32>> = vec![
            vec![-1, -1, 1, -1, -1],
            vec![1, 1, 1, 1, -1],
            vec![1, -1, 1, 1, -1],
            vec![1, -1, 1, 1, 1],
            vec![1, -1, 1, -1, 1],
            vec![-1, 1, 1, 1, 1],
            vec![-1, -1, 1, 1, 1],
            vec![1, 1, 1, -1, -1],
            vec![1, -1, -1, -1, -1],
            vec![1, 1, -1, -1, 1],
            vec![-1, -1, -1, 1, -1],
            vec![-1, 1, -1, 1, -1],
        ];

        let oxygen = super::extract_oxygen(example_case.clone());
        assert_eq!(super::count_to_binary_string(oxygen), "10111");

        let co2 = super::extract_co2(example_case);
        assert_eq!(super::count_to_binary_string(co2), "01010");
    }

}
