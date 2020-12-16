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

    let timestamp = input[0].parse::<u64>().unwrap();
    let mut min_wait = u64::MAX;
    let mut output: u64 = 0;
    for bus in input[1].split(',') {
        if bus == "x" {
            continue;
        }

        let bus_id = bus.parse::<u64>().unwrap();
        let wait = bus_id - (timestamp % bus_id);
        if wait < min_wait {
            min_wait = wait;
            output = wait * bus_id;
        }
    }
    println!("Part1: {}", output);
}

fn invmod(a: i128, b: i128) -> i128 {
    // returns a = integer, b = modulo

    if b == 1 {
        return 1;
    }

    let mut a = a;
    let mut b = b;
    let b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;

    while a > 1 {
        let q = a / b;

        let mut tmp = b;
        b = a % b;
        a = tmp;

        tmp = x0;
        x0 = x1 - q * x0;
        x1 = tmp;
    }

    if x1 < 0 {
        x1 += b0;
    }

    return x1;
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let input = input.unwrap();

    let mut buses: Vec<(i128, i128)> = Vec::new();

    let mut product: i128 = 1;

    for (i, bus) in input[1].split(',').enumerate() {
        if bus == "x" {
            continue;
        }

        let bus_id = bus.parse::<i128>().unwrap();

        product *= bus_id;

        buses.push((i as i128, bus_id));
    }

    let mut timestamp = 0;

    for (t, bus_id) in buses.iter() {
        let p = product / bus_id;
        timestamp += ((bus_id - t) % bus_id) * invmod(p, *bus_id) * p
    }
    timestamp %= product;

    println!("Part2: {}", timestamp);
}

fn main() {
    part1();
    part2();
}
