use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

    let input = input.unwrap();

    let w = input[0].len() as i64; // used as i64 to simplify overflow check later
    let h = input.len() as i64; // used as i64 to simplify overflow check later

    let mut waiting_area = String::with_capacity((w * h) as usize);
    for line in input.iter() {
        waiting_area.push_str(line);
    }

    let mut waiting_area: Vec<char> = waiting_area.chars().collect();
    loop {
        let mut new_waiting_area = waiting_area.clone();
        for y in 0..h {
            for x in 0..w {
                let index = (x + y * w) as usize;
                if waiting_area[index] == '.' {
                    // ignore floor spaces
                    continue;
                }

                let mut neighbours_count = 0;
                for i in 0..9 {
                    // check all neighbours
                    if i == 4 {
                        // 4 is + (0; 0)
                        continue;
                    }

                    let xx = x + (i / 3) - 1;
                    let yy = y + (i % 3) - 1;

                    if xx < 0 || yy < 0 || xx == w || yy == h {
                        continue;
                    }

                    let index = (xx + yy * w) as usize;

                    if waiting_area[index] == '#' {
                        neighbours_count += 1;
                    }
                }

                new_waiting_area[index] = match neighbours_count {
                    0 => {
                        if waiting_area[index] == 'L' {
                            '#'
                        } else {
                            waiting_area[index]
                        }
                    }
                    4..=8 => {
                        if waiting_area[index] == '#' {
                            'L'
                        } else {
                            waiting_area[index]
                        }
                    }
                    _ => waiting_area[index], // does not change
                }
            }
        }
        if waiting_area == new_waiting_area {
            waiting_area = new_waiting_area;
            break;
        }
        waiting_area = new_waiting_area;
    }

    let count = waiting_area.iter().filter(|&c| *c == '#').count();
    println!("Part1: {}", count);
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let input = input.unwrap();

    let w = input[0].len() as i64; // used as i64 to simplify overflow check later
    let h = input.len() as i64; // used as i64 to simplify overflow check later

    let mut waiting_area = String::with_capacity((w * h) as usize);
    for line in input.iter() {
        waiting_area.push_str(line);
    }

    let mut waiting_area: Vec<char> = waiting_area.chars().collect();
    loop {
        let mut new_waiting_area = waiting_area.clone();
        for y in 0..h {
            for x in 0..w {
                let index = (x + y * w) as usize;
                if waiting_area[index] == '.' {
                    // ignore floor spaces
                    continue;
                }

                let mut visible_occupied_seats = 0;
                for i in 0..9 {
                    // check all neighbours
                    if i == 4 {
                        // 4 is + (0; 0)
                        continue;
                    }

                    let step_x = (i / 3) - 1;
                    let step_y = (i % 3) - 1;

                    let mut xx = x + step_x;
                    let mut yy = y + step_y;
                    loop {
                        if xx < 0 || yy < 0 || xx == w || yy == h {
                            break;
                        }
                        let index = (xx + yy * w) as usize;
                        if waiting_area[index] == '#' {
                            visible_occupied_seats += 1;
                        }

                        if waiting_area[index] == '#' || waiting_area[index] == 'L' {
                            break;
                        }

                        xx += step_x;
                        yy += step_y;
                    }
                }

                new_waiting_area[index] = match visible_occupied_seats {
                    0 => {
                        if waiting_area[index] == 'L' {
                            '#'
                        } else {
                            waiting_area[index]
                        }
                    }
                    5..=8 => {
                        if waiting_area[index] == '#' {
                            'L'
                        } else {
                            waiting_area[index]
                        }
                    }
                    _ => waiting_area[index], // does not change
                }
            }
        }
        if waiting_area == new_waiting_area {
            waiting_area = new_waiting_area;
            break;
        }
        waiting_area = new_waiting_area;
    }

    let count = waiting_area.iter().filter(|&c| *c == '#').count();
    println!("Part2: {}", count);
}

fn main() {
    part1();
    part2();
}
