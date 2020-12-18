use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

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

    let mut active_cubes: HashSet<(i64, i64, i64)> = HashSet::new();

    for (y, line) in input.unwrap().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i64, y as i64, 0));
            }
        }
    }

    let mut cycle = 0;
    loop {
        if cycle == 6 {
            break;
        }

        let mut active_neighbours_map: HashMap<(i64, i64, i64), u64> = HashMap::new();

        for (x, y, z) in active_cubes.iter() {
            for i in 0..27 {
                if i == 13 {
                    // 13 => (0; 0; 0)
                    continue;
                }

                let neighbour = (
                    x + ((i % 9) % 3) - 1,
                    y + ((i % 9) / 3) - 1,
                    z + (i / 9) - 1,
                );

                *active_neighbours_map.entry(neighbour).or_insert(0) += 1
            }
        }

        let mut new_active_cubes: HashSet<(i64, i64, i64)> = HashSet::new();

        for (cube, active_neighbours) in active_neighbours_map.iter() {
            if active_cubes.contains(cube) {
                // cube is active
                if (2..=3).contains(active_neighbours) {
                    // and has 2 or 3 active neighbours
                    new_active_cubes.insert(*cube);
                }
            } else {
                // cube is inactive
                if *active_neighbours == 3 {
                    // and has exactly 3 active neighbours
                    new_active_cubes.insert(*cube);
                }
            }
        }

        // swap the active cubes
        active_cubes = new_active_cubes;
        // go to next cycle
        cycle += 1;
    }

    println!("Part1: {}", active_cubes.len());
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut active_cubes: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    //                              w    x    y    z

    for (y, line) in input.unwrap().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((0, x as i64, y as i64, 0));
            }
        }
    }

    let mut cycle = 0;
    loop {
        if cycle == 6 {
            break;
        }

        let mut active_neighbours_map: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

        for (w, x, y, z) in active_cubes.iter() {
            for ww in -1..=1 {
                for i in 0..27 {
                    if ww == 0 && i == 13 {
                        // (0; 0; 0; 0) // the current cube
                        continue;
                    }
                    let neighbour = (
                        w + ww,
                        x + ((i % 9) % 3) - 1,
                        y + ((i % 9) / 3) - 1,
                        z + (i / 9) - 1,
                    );
                    *active_neighbours_map.entry(neighbour).or_insert(0) += 1
                }
            }
        }
        let mut new_active_cubes: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        for (cube, active_neighbours) in active_neighbours_map.iter() {
            if active_cubes.contains(cube) {
                // cube is active
                if (2..=3).contains(active_neighbours) {
                    // and has 2 or 3 active neighbours
                    new_active_cubes.insert(*cube);
                }
            } else {
                // cube is inactive
                if *active_neighbours == 3 {
                    // and has exactly 3 active neighbours
                    new_active_cubes.insert(*cube);
                }
            }
        }

        // swap the active cubes
        active_cubes = new_active_cubes;
        // go to next cycle
        cycle += 1;
    }

    println!("Part2: {}", active_cubes.len());
}

fn main() {
    part1();
    part2();
}
