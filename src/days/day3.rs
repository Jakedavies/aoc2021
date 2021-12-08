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

pub fn part1() {
    let mut counts = vec![];

    if let Ok(lines) = file_utils::read_lines("./inputs/input3") {
        for line in lines {
            if let Ok(instruction_string) = line {
                counts = process_binary_string(counts, &instruction_string)
            }
        }
    }

    let mut result: String = "".to_owned();
    for count in counts {
        if count > 0 {
            result += "1";
        } else {
            result += "0";
        }
    }
    println!("{}", i32::from_str_radix(&*result, 2).unwrap());
    println!("{}", i32::from_str_radix(&invert_binary_string(&*result), 2).unwrap());
}



#[cfg(test)]
mod tests {
    #[test]
    fn process_character() {
        let mut counts = vec![];
        counts = super::process_binary_string(counts, "010100");

        assert_eq!(counts, vec![0,1,0,1,0,0])
    }

}
