use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<String>, io::Error> {
    let filename = "input.txt";

    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

fn part1() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut answers: HashSet<char> = HashSet::new();
    let mut total = 0;
    for line in input.unwrap().iter() {
        if line.is_empty() {
            total += answers.len();
            answers.clear();
        }

        for answer in line.chars() {
            answers.insert(answer);
        }
    }
    total += answers.len();
    println!("Part1: {}", total);
}

fn part2() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut answers: HashMap<char, usize> = HashMap::new();
    let mut total = 0;
    let mut persons_in_group = 0;
    for line in input.unwrap().iter() {
        if line.is_empty() {
            // count the amount of answer where answer_count == persons_in_group
            let answered_by_everyone = answers
                .iter()
                .filter(|&(_, count)| count == &persons_in_group)
                .count();
            total += answered_by_everyone;
            // reset the counters
            answers.clear();
            persons_in_group = 0;
            continue;
        }

        for answer in line.chars() {
            let count = answers.entry(answer).or_insert(0);
            *count += 1;
        }
        persons_in_group += 1;
    }
    let answered_by_everyone = answers
        .iter()
        .filter(|&(_, count)| count == &persons_in_group)
        .count();

    total += answered_by_everyone;

    println!("Part2: {}", total);
}

fn main() {
    part1();
    part2();
}
