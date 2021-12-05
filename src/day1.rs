pub fn task1(input: Vec<String>) {
    let mut inc: i32 = 0;
    let mut prev: i32 = -1;

    for i in input {
        let depth = i.parse::<i32>().unwrap();
        if prev != -1 && prev < depth {
            inc = inc + 1;
        }
        prev = depth;
    }

    println!("increases {}", inc);
}

pub fn task2(input: Vec<String>) {
    let ii: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut inc: i32 = 0;
    let mut prev: i32 = -1;
    for i in ii.windows(3) {
        let depth = i.iter().sum();
        if prev != -1 && prev < depth {
            inc = inc + 1;
        }
        prev = depth;
    }

    println!("increases {}", inc);
}
