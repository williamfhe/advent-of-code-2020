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

    let input = input.unwrap();

    let mut mask_0: u64 = (1 << 36) - 1; // will be used to &
    let mut mask_1: u64 = 0; // will be used to |

    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in input.iter() {
        let splitted_line: Vec<&str> = line.split(['=', ' ', '[', ']'].as_ref()).collect();

        match splitted_line[0] {
            "mask" => {
                let mask = splitted_line[3];

                mask_0 = u64::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap();
                mask_1 = u64::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap();
            }
            "mem" => {
                let mem_index = splitted_line[1].parse::<u64>().unwrap();

                let mut value = splitted_line[5].parse::<u64>().unwrap();
                value |= mask_1;
                value &= mask_0;

                memory.insert(mem_index, value);
            }
            _ => unreachable!(),
        }
    }

    let total: u64 = memory.values().sum();

    println!("Part1: {}", total);
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let input = input.unwrap();

    let mut mask: u64 = 0; // will be used to |

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut floating_bit_index: Vec<usize> = Vec::new();

    for line in input.iter() {
        let splitted_line: Vec<&str> = line.split(['=', ' ', '[', ']'].as_ref()).collect();

        match splitted_line[0] {
            "mask" => {
                mask = u64::from_str_radix(splitted_line[3].replace("X", "0").as_str(), 2).unwrap();
                floating_bit_index.clear();

                for (i, c) in splitted_line[3].chars().enumerate() {
                    if c == 'X' {
                        floating_bit_index.push(i);
                    }
                }
            }
            "mem" => {
                let value = splitted_line[5].parse::<u64>().unwrap();
                let base_mem_index = splitted_line[1].parse::<u64>().unwrap() | mask;

                let base_mem_index_str = format!("{:0>36b}", base_mem_index);

                for float in 0..(1 << floating_bit_index.len()) {
                    let mut base_mem_index_chars: Vec<char> = base_mem_index_str.chars().collect();

                    let floating_bits: Vec<char> =
                        format!("{:0>36b}", float).chars().rev().collect();

                    for (i, bit_index) in floating_bit_index.iter().enumerate() {
                        base_mem_index_chars[*bit_index] = floating_bits[i];
                    }

                    let insert_mem_index: String = base_mem_index_chars.into_iter().collect();

                    let insert_mem_index =
                        u64::from_str_radix(insert_mem_index.as_str(), 2).unwrap();

                    memory.insert(insert_mem_index, value);
                }
            }
            _ => unreachable!(),
        }
    }

    let total: u64 = memory.values().sum();

    println!("Part1: {}", total);
}

fn main() {
    part1();
    part2();
}
