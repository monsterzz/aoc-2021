mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::{env, fmt, io};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::BufRead;

struct Config {
    task: String,
    filename: String,
}

#[derive(Debug, Clone)]
struct ConfigError {
    prg: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse args into config")
    }
}

fn main() {
    println!("Advent of Code runner");

    let config = parse_config().expect("failed to parse args");
    let data = read_input(config.filename).expect("failed to read input");

    let input: Vec<String> = data.map(|x| x.unwrap()).collect();

    match config.task.as_str() {
        "day_1_1" => day1::task1(input),
        "day_1_2" => day1::task2(input),

        "day_2_1" => day2::task1(input),
        "day_2_2" => day2::task2(input),

        "day_3_1" => day3::task1(input),
        "day_3_2" => day3::task2(input),

        "day_4_1" => day4::task1(input),
        "day_4_2" => day4::task2(input),

        "day_5_1" => day5::task1(input),
        "day_5_2" => day5::task2(input),

        "day_6_1" => day6::task1(input),
        "day_6_2" => day6::task2(input),

        "day_7_1" => day7::task1(input),
        "day_7_2" => day7::task2(input),

        _ => panic!("unknown task"),
    }
}

fn read_input(filename: String) -> Result<Lines<BufReader<File>>, io::Error> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_config() -> Result<Config, ConfigError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Result::Err(ConfigError { prg: args[0].clone() });
    }

    let config = Config {
        task: args[1].clone(),
        filename: args[2].clone(),
    };

    Result::Ok(config)
}
