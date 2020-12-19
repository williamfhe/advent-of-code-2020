use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::VecDeque;

fn read_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

fn part1_generate_expression(
    broken_expression: &Vec<&str>,
    current_index: &mut usize,
) -> VecDeque<String> {
    let mut returned_expression: VecDeque<String> = VecDeque::new();

    while *current_index < broken_expression.len() {
        let composant = broken_expression[*current_index];

        match composant {
            ")" => {
                break;
            }
            "(" => {
                *current_index += 1;
                let parenthesis_expression =
                    part1_generate_expression(broken_expression, current_index);

                returned_expression.push_back(String::from("("));
                parenthesis_expression
                    .iter()
                    .for_each(|comp| returned_expression.push_back(comp.clone()));
                returned_expression.push_back(String::from(")"));
            }
            "+" => {
                returned_expression.push_back(composant.to_owned()); // +
            }
            "*" => {
                returned_expression.push_front(String::from("("));
                returned_expression.push_back(String::from(")"));
                returned_expression.push_back(composant.to_owned()); // *
            }
            _ => {
                returned_expression.push_back(composant.to_owned()); // is a number
            }
        }

        *current_index += 1;
    }

    returned_expression
}

fn part1() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut sum = 0;

    for line in input.unwrap().iter() {
        let mut raw_expression = line.to_owned().clone();
        raw_expression = raw_expression.replace('(', "( ");
        raw_expression = raw_expression.replace(')', " )");

        let broken_expression: Vec<&str> = raw_expression.split_whitespace().collect();
        let transformed_expression = part1_generate_expression(&broken_expression, &mut 0);

        let mut expression = String::new();
        transformed_expression
            .iter()
            .for_each(|comp| expression.push_str(comp));

        let mut ns = fasteval::EmptyNamespace;
        sum += fasteval::ez_eval(expression.as_str(), &mut ns).unwrap() as u64;
    }

    println!("Part1: {}", sum);
}

fn part2_generate_expression(
    broken_expression: &Vec<&str>,
    current_index: &mut usize,
) -> VecDeque<String> {
    let mut returned_expression: VecDeque<String> = VecDeque::new();
    let mut add_parts: VecDeque<String> = VecDeque::new();

    while *current_index < broken_expression.len() {
        let composant = broken_expression[*current_index];

        match composant {
            ")" => {
                break;
            }
            "(" => {
                *current_index += 1;
                let parenthesis_expression =
                    part2_generate_expression(broken_expression, current_index);

                add_parts.push_back(String::from("("));
                parenthesis_expression
                    .iter()
                    .for_each(|comp| add_parts.push_back(comp.clone()));
                add_parts.push_back(String::from(")"));
            }
            "+" => {
                add_parts.push_back(composant.to_owned()); // +
            }
            "*" => {
                if add_parts.len() > 1 {
                    returned_expression.push_back(String::from("("));
                    add_parts
                        .iter()
                        .for_each(|comp| returned_expression.push_back(comp.clone()));
                    returned_expression.push_back(String::from(")"));
                } else {
                    add_parts
                        .iter()
                        .for_each(|comp| returned_expression.push_back(comp.clone()));
                }
                returned_expression.push_back(composant.to_owned()); // *

                add_parts.clear();
            }
            _ => {
                add_parts.push_back(composant.to_owned()); // is a number
            }
        }

        *current_index += 1;
    }

    returned_expression.push_back(String::from("("));
    add_parts
        .iter()
        .for_each(|comp| returned_expression.push_back(comp.clone()));
    returned_expression.push_back(String::from(")"));

    returned_expression
}

fn part2() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut sum = 0;

    for line in input.unwrap().iter() {
        let mut raw_expression = line.to_owned().clone();
        raw_expression = raw_expression.replace('(', "( ");
        raw_expression = raw_expression.replace(')', " )");

        let broken_expression: Vec<&str> = raw_expression.split_whitespace().collect();
        let transformed_expression = part2_generate_expression(&broken_expression, &mut 0);

        let mut expression = String::new();
        transformed_expression
            .iter()
            .for_each(|comp| expression.push_str(comp));

        let mut ns = fasteval::EmptyNamespace;
        let res = fasteval::ez_eval(expression.as_str(), &mut ns).unwrap() as u64;

        sum += res;
    }

    println!("Part2: {}", sum);
}

fn main() {
    // Dirty code, the goal is to add parenthesis to change the precedence order
    // Before evaluating the expression
    part1();
    part2();
}
