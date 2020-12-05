use std::cmp;
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

    let mut highest_boarding_pass = -1;

    for line in input.unwrap().iter() {
        let mut row = 0;
        let mut col = 0;
        let mut row_range = 64;
        let mut col_range = 4;
        for c in line.chars() {
            match c {
                'F' => {
                    row_range = row_range / 2;
                }
                'B' => {
                    row += row_range;
                    row_range = row_range / 2;
                }
                'L' => {
                    col_range = col_range / 2;
                }
                'R' => {
                    col += col_range;
                    col_range = col_range / 2;
                }
                _ => {}
            }
        }

        highest_boarding_pass = cmp::max(highest_boarding_pass, row * 8 + col);
    }

    println!("Part1 : {}", highest_boarding_pass);
}

fn part2() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut boarding_pass: Vec<usize> = (0..1024).collect();

    let mut min_boarding_pass = 1024;
    let mut max_boarding_pass = 0;

    for line in input.unwrap().iter() {
        let mut row = 0;
        let mut col = 0;
        let mut row_range = 64;
        let mut col_range = 4;
        for c in line.chars() {
            match c {
                'F' => {
                    row_range = row_range / 2;
                }
                'B' => {
                    row += row_range;
                    row_range = row_range / 2;
                }
                'L' => {
                    col_range = col_range / 2;
                }
                'R' => {
                    col += col_range;
                    col_range = col_range / 2;
                }
                _ => {}
            }
        }

        let boarding_pass_id = row * 8 + col;
        min_boarding_pass = cmp::min(min_boarding_pass, boarding_pass_id);
        max_boarding_pass = cmp::max(max_boarding_pass, boarding_pass_id);

        let index = boarding_pass
            .iter()
            .position(|x| x == &boarding_pass_id)
            .unwrap();
        boarding_pass.remove(index);
    }

    boarding_pass.retain(|x| x > &min_boarding_pass && x < &max_boarding_pass);
    println!("Part2: {}", boarding_pass[0]);
}

fn main() {
    part1();
    part2();
}
