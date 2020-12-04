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

fn parse_line(line: &str) -> (usize, usize, char, &str) {
    let splitted_line: Vec<&str> = line.split([' ', ':', '-'].as_ref()).collect();
    let first_integer = splitted_line[0].parse::<usize>().unwrap();
    let second_integer = splitted_line[1].parse::<usize>().unwrap();
    let character = splitted_line[2].chars().nth(0).unwrap();
    let password = splitted_line[4];

    return (first_integer, second_integer, character, password);
}

fn part1() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut ok_count = 0;
    for line in input.unwrap().iter() {
        let (min, max, character, password) = parse_line(line);
        let count = password.chars().filter(|&c| c == character).count();

        if min <= count && count <= max {
            ok_count += 1;
        }
    }
    println!("Part 1: {}", ok_count);
}

fn part2() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut ok_count = 0;
    for line in input.unwrap().iter() {
        let (pos1, pos2, character, password) = parse_line(line);
        if (password.chars().nth(pos1 - 1).unwrap() == character)
            != (password.chars().nth(pos2 - 1).unwrap() == character)
        {
            ok_count += 1;
        }
    }
    println!("Part 2: {}", ok_count);
}

fn main() {
    part1();
    part2();
}
