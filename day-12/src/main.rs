use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

const EAST: (i64, i64) = (1, 0);
const SOUTH: (i64, i64) = (0, -1);
const WEST: (i64, i64) = (-1, 0);
const NORTH: (i64, i64) = (0, 1);

const ORIENTATIONS: [(i64, i64); 4] = [EAST, SOUTH, WEST, NORTH];

fn part1() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut current_orientation = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let input = input.unwrap();
    for line in input.iter() {
        let direction = line.chars().nth(0).unwrap();
        let value = &line.as_str()[1..].parse::<i64>().unwrap();

        // For 'L' we add 4 to current_orientation because if X < 0
        // then (X % 4) < 0 but X is an usize so it crashes
        match direction {
            'R' => current_orientation = (current_orientation + (*value as usize / 90)) % 4,
            'L' => current_orientation = ((current_orientation + 4) - (*value as usize / 90)) % 4,
            _ => {
                let current_direction = match direction {
                    'E' => EAST,
                    'S' => SOUTH,
                    'W' => WEST,
                    'N' => NORTH,
                    'F' => ORIENTATIONS[current_orientation],
                    _ => unreachable!(),
                };

                let (mx, my) = current_direction;
                x += mx * value;
                y += my * value;
            }
        }
    }

    let dist = x.abs() + y.abs();
    println!("Part1: {}", dist);
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let (mut w_x, mut w_y): (i64, i64) = (10, 1);
    let (mut x, mut y): (i64, i64) = (0, 0);

    let input = input.unwrap();
    for line in input.iter() {
        let direction = line.chars().nth(0).unwrap();
        let value = &line.as_str()[1..].parse::<i64>().unwrap();

        match direction {
            'R' => {
                // simplified trigonometry
                for _ in 0..((value / 90) % 4) {
                    let w_x2 = w_y;
                    let w_y2 = -w_x;

                    w_x = w_x2;
                    w_y = w_y2;
                }
            }
            'L' => {
                // simplified trigonometry
                for _ in 0..((value / 90) % 4) {
                    let w_x2 = -w_y;
                    let w_y2 = w_x;

                    w_x = w_x2;
                    w_y = w_y2;
                }
            }
            _ => {
                match direction {
                    'E' => w_x += value,
                    'S' => w_y -= value,
                    'W' => w_x -= value,
                    'N' => w_y += value,
                    'F' => {
                        x += w_x * value;
                        y += w_y * value;
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    let dist = x.abs() + y.abs();
    println!("Part2: {}", dist);
}

fn main() {
    part1();
    part2();
}
