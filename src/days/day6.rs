use std::collections::HashMap;


pub fn simulate(fishes: &mut Vec<u64>) {
    let mut new_fish: Vec<u64> = vec![];
    for index in 0..fishes.len() {
        let f = &mut fishes[index];
        if *f == 0 {
           new_fish.push(8); 
           fishes[index] = 6;
        } else {
           fishes[index] = *f -1;
        }
    } 


    fishes.append(&mut new_fish);

    //println!("{:?}", fishes)
}

fn init_fishmap() -> HashMap<u8,u64> {
    let mut fish_counts = HashMap::<u8,u64>::new();

    // init the relevant counts
    for i in 0..8 {
        fish_counts.insert(i, 0);
    }
    fish_counts
}


pub fn simulate_v2(fishes: HashMap<u8, u64>) -> HashMap<u8, u64> {
    let mut new_fish = HashMap::<u8, u64>::new();
    for i in 0..8 {
        new_fish.insert(i, 0);
    }
    for (key, value) in fishes {
        match key {
            0 => {
                new_fish.insert(8, value);
                new_fish.insert(6, new_fish.get(&6).unwrap() + value);
            },
            7 => {
                new_fish.insert(6, new_fish.get(&6).unwrap() + value);
            },
            1 | 2 | 3 | 4 | 5 | 6 |  8 => {
                new_fish.insert(key-1, value);
            },
            _ => panic!("who let this fish in here")
        }
    }
    new_fish
}

pub fn part1() {
    let lines: Vec<&str> = include_str!("./../../inputs/input6")
        .lines()
        .collect();

    let mut fish: Vec<u64> = lines[0]
        .split(',')
        .map(|f| u64::from_str_radix(f, 10).unwrap())
        .collect();


    for _ in 0..80 {
        simulate(&mut fish);
    }

    println!("Finished simulating, {}", fish.len())
}

pub fn part2() {
    let lines: Vec<&str> = include_str!("./../../inputs/input6")
        .lines()
        .collect();

    let mut fish: Vec<u8> = lines[0]
        .split(',')
        .map(|f| u8::from_str_radix(f, 10).unwrap())
        .collect();

    let mut fish_counts = HashMap::<u8,u64>::new();

    // init the relevant counts
    for i in 0..8 {
        fish_counts.insert(i, 0);
    }
    

    for f in fish {
        let c = fish_counts.get_mut(&f).unwrap();
        *c = *c + 1;
    }

    for _ in 0..256 {
        fish_counts = simulate_v2(fish_counts);
    }


    let sum: u64 = fish_counts.values().sum();
    println!("Finished simulating, {}", sum)
}
