use std::str::FromStr;

fn parse_input(input: Vec<String>) -> Vec<i32> {
    return input.first().unwrap()
        .split(",")
        .map(|s| i32::from_str(s).unwrap())
        .collect::<Vec<i32>>();
}

fn fuel_to_position(data: &Vec<i32>, pos: i32) -> i32 {
    return data.iter()
        .map(|x| (*x - pos).abs())
        .sum::<i32>();
}

fn var_fuel_to_position(data: &Vec<i32>, pos: i32) -> i32 {
    return data.iter()
        .map(|x| {
            let n = (*x - pos).abs();
            return (n * (n + 1)) / 2;
        })
        .sum::<i32>();
}

pub fn task1(input: Vec<String>) {
    let data = parse_input(input);

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    let min_fuel = (min..max)
        .map(|x| fuel_to_position(&data, x))
        .min()
        .unwrap();

    println!("fuel={}", min_fuel);
}

pub fn task2(input: Vec<String>) {
    let data = parse_input(input);

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    let min_fuel = (min..max)
        .map(|x| var_fuel_to_position(&data, x))
        .min()
        .unwrap();

    println!("fuel={}", min_fuel);
}
