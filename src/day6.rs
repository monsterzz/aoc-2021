use std::str::FromStr;

fn parse_data(input: Vec<String>) -> Vec<u32> {
    return input.first().unwrap()
        .split(",")
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();
}

fn fish_days(data: Vec<u32>, days: i32) -> usize {
    let mut mc: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for x in data {
        mc[x as usize] += 1;
    }

    for _ in 0..days {
        let new = mc[0];
        mc[0] = 0;

        for i in 1..9 {
            mc[i - 1] = mc[i];
        }

        mc[6] += new;
        mc[8] = new;
    }

    return mc.iter().map(|v| *v).sum();
}

pub fn task1(input: Vec<String>) {
    println!("fish count {}", fish_days(parse_data(input), 80));
}

pub fn task2(input: Vec<String>) {
    println!("fish count {}", fish_days(parse_data(input), 256));
}
