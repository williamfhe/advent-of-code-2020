use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

fn find_first_invalid_number(numbers: &Vec<u64>) -> Option<u64> {
    let mut composant_numbers: VecDeque<u64> = VecDeque::new();

    for i in 0..25 {
        composant_numbers.push_front(numbers[i]);
    }

    for i in 25..numbers.len() {
        let current_number = numbers[i];

        let mut is_valid_number = false;
        // check the numbers in composant_numbers
        'check_loop: for x1 in 0..25 {
            for x2 in (x1 + 1)..25 {
                if composant_numbers[x1] + composant_numbers[x2] == current_number {
                    is_valid_number = true;
                    break 'check_loop;
                }
            }
        }

        if !is_valid_number {
            return Some(current_number);
        }

        composant_numbers.push_front(current_number);
        composant_numbers.pop_back();
    }

    None
}

fn part1() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let numbers: Vec<u64> = input
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let first_invalid_number = find_first_invalid_number(&numbers);

    if let None = first_invalid_number {
        eprintln!("Part1: Did not find the invalid number...");
        std::process::exit(1);
    } else {
        println!("Part1: {}", first_invalid_number.unwrap());
    }
}

fn part2_find_range(numbers: &Vec<u64>, wanted_sum: u64) -> Option<(usize, usize)> {
    for x1 in 0..numbers.len() {
        if numbers[x1] == wanted_sum {
            continue;
        }

        let mut sum = numbers[x1];

        for x2 in (x1 + 1)..numbers.len() {
            sum += numbers[x2];

            if sum > wanted_sum {
                // we went over the number
                break;
            }

            if sum == wanted_sum {
                // we found the range
                return Some((x1, x2));
            }
        }
    }
    None
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let numbers: Vec<u64> = input
        .unwrap()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let first_invalid_number = find_first_invalid_number(&numbers);

    if let None = first_invalid_number {
        eprintln!("Part2: Did not find the invalid number...");
        std::process::exit(1);
    }

    let first_invalid_number = first_invalid_number.unwrap();

    let range = part2_find_range(&numbers, first_invalid_number);
    if let None = range {
        eprintln!(
            "Part2: Did not find the range for {}...",
            first_invalid_number
        );
        std::process::exit(1);
    }
    let (range_start, range_end) = range.unwrap();

    let range = &numbers[range_start..=range_end];

    let weakness = range.iter().min().unwrap() + range.iter().max().unwrap();
    println!("Part2: {}", weakness);
}

fn main() {
    part1();
    part2();
}
