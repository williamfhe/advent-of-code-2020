use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::HashSet;

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

    let mut accumulator = 0;
    let mut instructions: Vec<(String, i64)> = Vec::new();

    for line in input.unwrap().iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        instructions.push((
            splitted_line[0].to_string(),
            splitted_line[1].parse::<i64>().unwrap(),
        ));
    }

    let mut used_instructions: HashSet<usize> = HashSet::new();

    let mut pointer: usize = 0;
    while !used_instructions.contains(&pointer) {
        used_instructions.insert(pointer);
        let (operation, argument) = &instructions[pointer as usize];
        match operation.as_str() {
            "jmp" => {
                pointer = (pointer as i64 + argument) as usize;
            }
            "nop" => {
                pointer += 1;
            }
            "acc" => {
                pointer += 1;
                accumulator += argument;
            }
            _ => {
                eprintln!(
                    "Unknown instruction, pointer={}, operation={}, argument={}",
                    pointer, operation, argument
                );
                std::process::exit(1);
            }
        }
    }

    println!("Part1: {}", accumulator);
}

fn part2_execute_program(
    instructions: &Vec<(String, i64)>,
    used_instructions: &mut HashSet<usize>,
    accumulator: i64,
    pointer: usize,
    can_do_recursion: bool,
) -> (i64, bool) {
    // when we encounter a nop or a jmp, we check if the program ends gracefully
    // when we swap it with the other instruction

    let mut accumulator = accumulator;
    let mut pointer = pointer;

    while !used_instructions.contains(&pointer) {
        if pointer >= instructions.len() {
            // graceful end
            return (accumulator, true);
        }

        used_instructions.insert(pointer);
        let (operation, argument) = &instructions[pointer];

        match operation.as_str() {
            "jmp" => {
                if *argument != 0 && can_do_recursion {
                    // act as a nop
                    let (acc, graceful_end) = part2_execute_program(
                        instructions,
                        &mut used_instructions.clone(),
                        accumulator,
                        pointer + 1,
                        false,
                    );
                    if graceful_end {
                        return (acc, true);
                    }
                }

                // stay as a jmp
                pointer = (pointer as i64 + argument) as usize;
            }
            "nop" => {
                if *argument != 0 && can_do_recursion {
                    // act as a jmp
                    let (acc, graceful_end) = part2_execute_program(
                        instructions,
                        &mut used_instructions.clone(),
                        accumulator,
                        (pointer as i64 + argument) as usize,
                        false,
                    );
                    if graceful_end {
                        return (acc, true);
                    }
                }

                // stay as a nop
                pointer += 1;
            }
            "acc" => {
                pointer += 1;
                accumulator += argument;
            }
            _ => {
                eprintln!(
                    "Unknown instruction, pointer={}, operation={}, argument={}",
                    pointer, operation, argument
                );
                std::process::exit(1);
            }
        }
    }

    (-1, false) // should never be reached
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut instructions: Vec<(String, i64)> = Vec::new();
    for line in input.unwrap().iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        instructions.push((
            splitted_line[0].to_string(),
            splitted_line[1].parse::<i64>().unwrap(),
        ));
    }

    let mut used_instructions: HashSet<usize> = HashSet::new();

    let (accumulator, graceful_end) =
        part2_execute_program(&instructions, &mut used_instructions, 0, 0, true);

    if !graceful_end {
        println!("Non graceful end!");
    }

    println!("Part2: {}", accumulator);
}

fn main() {
    part1();
    part2();
}
