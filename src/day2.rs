pub fn task1(input: Vec<String>) {
    let mut hp: i32 = 0;
    let mut depth: i32 = 0;

    for line in input {
        let delimiter = line.find(' ').unwrap();
        let command = &line[0..delimiter];
        let quantifier = &line[(delimiter + 1)..].parse::<i32>().unwrap();

        match command {
            "forward" => hp += quantifier,
            "down" => depth += quantifier,
            "up" => depth -= quantifier,
            _ => panic!("unknown command {}", command)
        }
    }

    println!("hp={}, depth={}, mult={}", hp, depth, hp * depth)
}

pub fn task2(input: Vec<String>) {
    let mut hp: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in input {
        let delimiter = line.find(' ').unwrap();
        let command = &line[0..delimiter];
        let quantifier = &line[(delimiter + 1)..].parse::<i32>().unwrap();

        match command {
            "forward" => {
                hp += quantifier;
                depth += quantifier * aim;
            }
            "down" => aim += quantifier,
            "up" => aim -= quantifier,
            _ => panic!("unknown command {}", command)
        }
    }

    println!("hp={}, depth={}, mult={}", hp, depth, hp * depth)
}
