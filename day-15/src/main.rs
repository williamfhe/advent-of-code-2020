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

    let mut number_memory: HashMap<usize, usize> = HashMap::new();
    let mut current_turn: usize = 0;
    let mut next_number: usize = 0;

    for number_str in input.unwrap()[0].split(',') {
        current_turn += 1;
        let number = number_str.parse::<usize>().unwrap();

        next_number = match number_memory.get(&number) {
            Some(last_turn) => current_turn - last_turn,
            None => 0,
        };

        number_memory.insert(number, current_turn);
    }

    loop {
        current_turn += 1;
        if current_turn == 2020 {
            break;
        }

        let number = next_number;

        next_number = match number_memory.get(&number) {
            Some(last_turn) => current_turn - last_turn,
            None => 0,
        };

        number_memory.insert(number, current_turn);
    }

    println!("Part1: {}", next_number);
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut number_memory: HashMap<usize, usize> = HashMap::new();
    let mut current_turn: usize = 0;
    let mut next_number: usize = 0;

    for number_str in input.unwrap()[0].split(',') {
        current_turn += 1;
        let number = number_str.parse::<usize>().unwrap();

        next_number = match number_memory.get(&number) {
            Some(last_turn) => current_turn - last_turn,
            None => 0,
        };

        number_memory.insert(number, current_turn);
    }

    loop {
        current_turn += 1;
        if current_turn == 30000000 {
            break;
        }

        let number = next_number;

        next_number = match number_memory.get(&number) {
            Some(last_turn) => current_turn - last_turn,
            None => 0,
        };

        number_memory.insert(number, current_turn);
    }

    println!("Part2: {}", next_number);
}

fn main() {
    part1();
    part2();
}
