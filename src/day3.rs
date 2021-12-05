fn bits_to_isize(b: &Vec<i32>, f: impl FnMut(&i32) -> &str) -> isize {
    return isize::from_str_radix(bits_to_chars(b, f)
                                     .join("")
                                     .as_str()
                                 , 2).unwrap();
}

fn bits_to_chars(b: &Vec<i32>, f: impl FnMut(&i32) -> &str) -> Vec<String> {
    return b.iter()
        .map(f)
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
}

fn most_common_bit(size: usize) -> impl FnMut(&i32) -> &str {
    let half = (f64::from(size as i32) / 2.0).ceil() as i32;
    move |x| {
        if *x >= half {
            "1"
        } else {
            "0"
        }
    }
}

fn least_common_bit(size: usize) -> impl FnMut(&i32) -> &str {
    let half = (f64::from(size as i32) / 2.0).ceil() as i32;
    move |x| {
        if *x < half {
            "1"
        } else {
            "0"
        }
    }
}

fn count_bits(input: &Vec<String>) -> Vec<i32> {
    let mut bit_counter = Vec::new();

    for line in input {
        if bit_counter.len() < line.len() {
            bit_counter.resize(line.len(), 0);
        }

        for (pos, x) in line.char_indices() {
            if x == '1' {
                bit_counter[pos] += 1;
            }
        }
    }

    return bit_counter;
}

pub fn task1(input: Vec<String>) {
    let len = input.len();
    let bit_counter = count_bits(&input);

    let gamma = bits_to_isize(&bit_counter, most_common_bit(len));
    let epsilon = bits_to_isize(&bit_counter, least_common_bit(len));

    println!("gamma={}, epsilon={}", gamma, epsilon);
    println!("result={}", gamma * epsilon);
}

pub fn task2(input: Vec<String>) {
    let len = input.len();

    let mut oxy_values = input.clone();
    let mut co2_values = input.clone();

    for i in 0..len {
        if oxy_values.len() == 1 && co2_values.len() == 1{
            break;
        }

        if oxy_values.len() > 1 {
            let bit_counter = count_bits(&oxy_values);
            let bits = bits_to_chars(&bit_counter, most_common_bit(oxy_values.len()));
            let bit = bits[i].chars().nth(0).unwrap();
            oxy_values = oxy_values.into_iter().filter(|v| {
                v.chars().nth(i).unwrap() == bit
            }).collect();
        }

        if co2_values.len() > 1 {
            let bit_counter = count_bits(&co2_values);
            let bits = bits_to_chars(&bit_counter, least_common_bit(co2_values.len()));
            let bit = bits[i].chars().nth(0).unwrap();
            co2_values = co2_values.into_iter().filter(|v| {
                v.chars().nth(i).unwrap() == bit
            }).collect();
        }
    }

    let oxygen = isize::from_str_radix(oxy_values[0].clone().as_str(), 2).unwrap();
    let co2 = isize::from_str_radix(co2_values[0].clone().as_str(), 2).unwrap();

    println!("oxygen={}, co2={}", oxygen, co2);
    println!("result={}", oxygen * co2);
}
