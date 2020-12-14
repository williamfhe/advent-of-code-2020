use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn read_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

fn part1() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut adapters: Vec<u64> = input
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    adapters.sort();

    let mut one_jolt_step = 0;
    let mut three_jolt_step = 0;

    let mut last_adapter = 0;
    for adapter in adapters.iter() {
        match adapter - last_adapter {
            1 => one_jolt_step += 1,
            3 => three_jolt_step += 1,
            _ => {}
        }
        last_adapter = *adapter;
    }

    three_jolt_step += 1;

    println!("Part1: {}", one_jolt_step * three_jolt_step);
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut adapters: Vec<u64> = input
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    adapters.sort();

    let mut combinations: HashMap<u64, u64> = HashMap::new();
    for i in 0..3 {
        if adapters[i] < 4 {
            combinations.insert(adapters[i], 1);
        } else {
            break;
        }
    }

    for adapter in adapters.iter() {
        let combinations_count = combinations.get(adapter).unwrap().clone();

        for offset in 1..=3 {
            *combinations.entry(adapter + offset).or_insert(0) += combinations_count;
        }
    }

    let total_combinations_count = combinations.get(adapters.last().unwrap()).unwrap();
    println!("Part2: {}", total_combinations_count);
}

fn main() {
    part1();
    part2();
}
