use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<i32>, io::Error> {
    let filename = "input.txt";

    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let input: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    Ok(input)
}

fn part_1() -> i32 {
    let input = read_input();

    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let values = input.unwrap();

    for val in values.iter() {
        let wanted_value = 2020 - val;
        if values.contains(&wanted_value) {
            return val * wanted_value;
        }
    }

    return 0;
}

fn part_2() -> i32 {
    let input = read_input();

    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let values = input.unwrap();

    for (i, val1) in values.iter().enumerate() {
        for j in i + 1..values.len() {
            let val2 = values[j];
            let wanted_value = 2020 - val1 - val2;
            if values.contains(&wanted_value) {
                return val1 * val2 * wanted_value;
            }
        }
    }

    return 0;
}

fn main() {
    let part1_result = part_1();
    println!("Result for part1 is: {}", part1_result);
    let part2_result = part_2();
    println!("Result for part2 is: {}", part2_result);
}
