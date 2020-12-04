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

    const WANTED_FIELDS: u32 = 7;

    let mut current_password_fields = 0;
    let mut valid_passwords = 0;

    for line in input.unwrap().iter() {
        if line.is_empty() {
            if current_password_fields == WANTED_FIELDS {
                valid_passwords += 1;
            }
            current_password_fields = 0;
            continue;
        }

        let splitted_line: Vec<&str> = line.split([' ', ':'].as_ref()).collect();
        current_password_fields += (splitted_line.len() / 2) as u32;

        if splitted_line.contains(&"cid") {
            // if the line contains cid, we decrease by one since we ignore this field
            current_password_fields -= 1;
        }
    }

    if current_password_fields == WANTED_FIELDS {
        valid_passwords += 1;
    }

    println!("Part1 : {}", valid_passwords);
}

fn is_field_valid(field_name: &str, field_value: &str) -> bool {
    return match field_name {
        "byr" => match field_value.parse::<u32>() {
            Ok(year) => match year {
                1920..=2002 => true,
                _ => false,
            },
            Err(_) => false,
        },
        "iyr" => match field_value.parse::<u32>() {
            Ok(year) => match year {
                2010..=2020 => true,
                _ => false,
            },
            Err(_) => false,
        },
        "eyr" => match field_value.parse::<u32>() {
            Ok(year) => match year {
                2020..=2030 => true,
                _ => false,
            },
            Err(_) => false,
        },
        "hgt" => {
            if field_value.ends_with("cm") {
                match field_value.strip_suffix("cm").unwrap().parse::<u32>() {
                    Ok(height) => match height {
                        150..=193 => true,
                        _ => false,
                    },
                    Err(_) => false,
                }
            } else if field_value.ends_with("in") {
                match field_value.strip_suffix("in").unwrap().parse::<u32>() {
                    Ok(height) => match height {
                        59..=76 => true,
                        _ => false,
                    },
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        "hcl" => {
            if field_value.starts_with('#') {
                match hex::decode(&field_value[1..]) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        "ecl" => match field_value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => field_value.len() == 9 && field_value.parse::<u32>().is_ok(),
        _ => false,
    };
}

fn assert_fields_valid() {
    assert_eq!(is_field_valid("byr", "1919"), false);
    assert_eq!(is_field_valid("byr", "1920"), true);
    assert_eq!(is_field_valid("byr", "2002"), true);
    assert_eq!(is_field_valid("byr", "2003"), false);
    assert_eq!(is_field_valid("byr", "text"), false);

    assert_eq!(is_field_valid("iyr", "2009"), false);
    assert_eq!(is_field_valid("iyr", "2010"), true);
    assert_eq!(is_field_valid("iyr", "2020"), true);
    assert_eq!(is_field_valid("iyr", "2021"), false);
    assert_eq!(is_field_valid("iyr", "text"), false);

    assert_eq!(is_field_valid("eyr", "2019"), false);
    assert_eq!(is_field_valid("eyr", "2020"), true);
    assert_eq!(is_field_valid("eyr", "2030"), true);
    assert_eq!(is_field_valid("eyr", "2031"), false);
    assert_eq!(is_field_valid("eyr", "text"), false);

    assert_eq!(is_field_valid("hgt", "149cm"), false);
    assert_eq!(is_field_valid("hgt", "150cm"), true);
    assert_eq!(is_field_valid("hgt", "193cm"), true);
    assert_eq!(is_field_valid("hgt", "194cm"), false);
    assert_eq!(is_field_valid("hgt", "58in"), false);
    assert_eq!(is_field_valid("hgt", "59in"), true);
    assert_eq!(is_field_valid("hgt", "76in"), true);
    assert_eq!(is_field_valid("hgt", "77in"), false);
    assert_eq!(is_field_valid("hgt", "textcm"), false);
    assert_eq!(is_field_valid("hcl", "#123abc"), true);
    assert_eq!(is_field_valid("hcl", "#123abz"), false);

    assert_eq!(is_field_valid("pid", "0123456789"), false);
    assert_eq!(is_field_valid("pid", "000000001"), true);
    assert_eq!(is_field_valid("pid", "text"), false);

    assert_eq!(is_field_valid("ecl", "brn"), true);
    assert_eq!(is_field_valid("ecl", "wat"), false);
}

fn part2() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    const WANTED_FIELDS: u32 = 7;

    let mut current_password_fields = 0;
    let mut valid_passwords = 0;

    for line in input.unwrap().iter() {
        if line.is_empty() {
            if current_password_fields == WANTED_FIELDS {
                valid_passwords += 1;
            }
            current_password_fields = 0;
            continue;
        }

        let splitted_line: Vec<&str> = line.split(' ').collect();

        // iter over fields contained in the line
        for field in splitted_line.iter() {
            let splitted_field: Vec<&str> = field.split(':').collect();
            if splitted_field[0] == "cid" {
                // ignore the field "cid"
                continue;
            }

            if is_field_valid(splitted_field[0], splitted_field[1]) {
                current_password_fields += 1;
            }
        }
    }

    if current_password_fields == WANTED_FIELDS {
        valid_passwords += 1;
    }

    println!("Part2 : {}", valid_passwords);
}

fn main() {
    assert_fields_valid();
    part1();
    part2();
}
