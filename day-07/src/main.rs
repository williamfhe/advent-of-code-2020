use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

fn read_input() -> Result<Vec<String>, io::Error> {
    let filename = "input.txt";

    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

fn part1_count_bags(
    current_bag: &str,
    bag_map: &HashMap<String, Vec<String>>,
    checked_bags: &mut HashSet<String>,
) -> u32 {
    if !bag_map.contains_key(current_bag) {
        return 1;
    }
    let mut total_bags = 1;
    for bag in bag_map.get(current_bag).unwrap().iter() {
        if checked_bags.contains(bag) {
            continue;
        }
        total_bags += part1_count_bags(bag, bag_map, checked_bags);
        checked_bags.insert(bag.clone());
    }
    return total_bags;
}

fn part1() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let mut bag_contained_by: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.unwrap().iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        let parent_bag = format!("{} {}", splitted_line[0], splitted_line[1]);

        if splitted_line[4] == "no" {
            // contain no other bags.
            continue;
        }

        let mut i = 4;
        while i < splitted_line.len() {
            let current_bag = format!("{} {}", splitted_line[i + 1], splitted_line[i + 2]);
            let current_bag_contained_by = bag_contained_by.entry(current_bag).or_insert(vec![]);
            current_bag_contained_by.push(parent_bag.clone());
            i += 4;
        }
    }

    let mut checked_bags: HashSet<String> = HashSet::new();
    // minus one because it counts the shiny gold one
    let count = part1_count_bags("shiny gold", &bag_contained_by, &mut checked_bags) - 1;

    println!("Part1: {}", count);
}

fn part2_count_bags(
    current_bag: &str,
    bag_can_contain: &HashMap<String, Vec<(String, u64)>>,
    bag_content_count: &mut HashMap<String, u64>,
) -> u64 {
    if bag_content_count.contains_key(current_bag) {
        return *bag_content_count.get(current_bag).unwrap();
    }

    if !bag_can_contain.contains_key(current_bag) {
        bag_content_count.insert(current_bag.to_string(), 0);
        return 0;
    }

    let mut total_child_bags = 0;
    for (child_bag, quantity) in bag_can_contain.get(current_bag).unwrap().iter() {
        let child_bag_count = part2_count_bags(child_bag, bag_can_contain, bag_content_count);
        total_child_bags += quantity * (child_bag_count + 1);
    }

    bag_content_count.insert(current_bag.to_string(), total_child_bags);

    return total_child_bags;
}

fn part2() {
    let input = read_input();
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }
    // which bag contains what
    let mut bag_can_contain: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    // used to avoid a recount of the child bags
    let mut bad_content_count: HashMap<String, u64> = HashMap::new();

    for line in input.unwrap().iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        let parent_bag = format!("{} {}", splitted_line[0], splitted_line[1]);

        if splitted_line[4] == "no" {
            // contain no other bags.
            bad_content_count.insert(parent_bag, 0);
            continue;
        }

        let mut bag_content: Vec<(String, u64)> = Vec::new();
        let mut i = 4;
        while i < splitted_line.len() {
            let bag_count = splitted_line[i].parse::<u64>().unwrap();
            let child_bag = format!("{} {}", splitted_line[i + 1], splitted_line[i + 2]);
            bag_content.push((child_bag, bag_count));
            i += 4;
        }
        bag_can_contain.insert(parent_bag, bag_content);
    }

    let bag_count = part2_count_bags("shiny gold", &bag_can_contain, &mut bad_content_count);

    println!("Part2: {}", bag_count);
}

fn main() {
    part1();
    part2();
}
