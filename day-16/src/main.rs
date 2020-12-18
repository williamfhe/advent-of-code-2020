use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Field {
    name: String,
    first_range: RangeInclusive<u64>,
    second_range: RangeInclusive<u64>,
    possibles_indexes: HashSet<usize>,
}

impl Field {
    fn build_field(
        name: String,
        first_range: RangeInclusive<u64>,
        second_range: RangeInclusive<u64>,
    ) -> Field {
        // creates and returns a new Field instance
        Field {
            name,
            first_range,
            second_range,
            possibles_indexes: Default::default(),
        }
    }

    fn match_number(&self, number: u64) -> bool {
        // returns true if number matches any of the field's ranges.
        self.first_range.contains(&number) || self.second_range.contains(&number)
    }

    fn init_possibles_indexes(&mut self, field_count: usize) {
        // initialize the field's hashset.
        self.possibles_indexes = (0..field_count).collect();
    }

    fn index_intersection(&mut self, possible_indexes: &HashSet<usize>) {
        self.possibles_indexes = self
            .possibles_indexes
            .intersection(possible_indexes)
            .map(|i| *i)
            .collect();
    }

    fn index_difference(&mut self, index_to_remove: &HashSet<usize>) {
        self.possibles_indexes = self
            .possibles_indexes
            .difference(index_to_remove)
            .map(|i| *i)
            .collect();
    }
}

fn read_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    Ok(input)
}

// returns the ticket as a vector of number (u64)
fn parse_ticket(ticket_line: &str) -> Vec<u64> {
    ticket_line
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn parse_fields(input: &Vec<String>, line_index: &mut usize) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    while *line_index < input.len() {
        let line = &input[*line_index];
        if line == "" {
            // we are at the end of fields declaration.
            *line_index += 2; // go to my ticket
            break;
        }

        let splitted_line: Vec<&str> = line.split(": ").collect();
        let field_range_str: Vec<&str> = splitted_line[1]
            .split([' ', 'o', 'r', '-'].as_ref())
            .collect();

        // parse the field's ranges
        let first_range_start = field_range_str[0].parse::<u64>().unwrap();
        let first_range_end = field_range_str[1].parse::<u64>().unwrap();
        let second_range_start = field_range_str[5].parse::<u64>().unwrap();
        let second_range_end = field_range_str[6].parse::<u64>().unwrap();

        // creates a new field
        fields.push(Field::build_field(
            splitted_line[0].to_owned(),
            first_range_start..=first_range_end,
            second_range_start..=second_range_end,
        ));

        *line_index += 1;
    }

    fields
}

fn analyze_nearby_tickets(
    fields: &mut Vec<Field>,
    input: &Vec<String>,
    line_index: &mut usize,
) -> u64 {
    let mut err_rate = 0;
    while *line_index < input.len() {
        // parse the ticket a vector of numbers (u64)
        let ticket = parse_ticket(&input[*line_index]);

        // used to find possible indexes for each field
        let mut field_possible_indexes_on_ticket: Vec<HashSet<usize>> = Vec::new();
        for _ in 0..fields.len() {
            field_possible_indexes_on_ticket.push(HashSet::new());
        }

        // check if ticket is valid
        let mut is_ticket_valid = true;
        for (number_index, number) in ticket.iter().enumerate() {
            // used to check if the number (and the ticket) is valid for the error_rate
            let mut is_number_valid = false;
            for (field_index, field) in fields.iter().enumerate() {
                // check if the field can be at the number's index
                if field.match_number(*number) {
                    is_number_valid = true;
                    field_possible_indexes_on_ticket[field_index].insert(number_index);
                }
            }

            if !is_number_valid {
                // invalid ticket, we increate err_rate skip this ticket
                is_ticket_valid = false;
                err_rate += number;
                break;
            }
        }

        if is_ticket_valid {
            // the ticket is valid, we filter the possible fields indexes
            for (field_index, field) in fields.iter_mut().enumerate() {
                field.index_intersection(&field_possible_indexes_on_ticket[field_index]);
            }
        }

        *line_index += 1;
    }

    err_rate
}

fn deduce_field_order(fields: &mut Vec<Field>) -> HashMap<String, usize> {
    // use to determine the real field index by deduction
    let mut field_order: HashMap<String, usize> = HashMap::new();

    // contains the used indexes
    let mut indexes_to_remove: HashSet<usize> = HashSet::new();

    loop {
        for field in fields.iter_mut() {
            if field_order.contains_key(&field.name) {
                // we already found the index of this field
                continue;
            }

            // remove from the field the indexes already attributed to other fields
            field.index_difference(&indexes_to_remove);

            match field.possibles_indexes.len() {
                0 => unreachable!(), // an error occured, the field has no possible index
                1 => {
                    let field_index = field.possibles_indexes.iter().next().unwrap();
                    field_order.insert(field.name.clone(), *field_index);
                    indexes_to_remove.insert(*field_index);
                }
                _ => {}
            }
        }

        if field_order.len() == fields.len() {
            // check if have found the order for each field
            break;
        }
    }

    field_order
}

fn calculate_departure_product(my_ticket: Vec<u64>, field_order: &HashMap<String, usize>) -> u64 {
    let mut departure_product = 1;
    for (field_name, field_index) in field_order.iter() {
        if field_name.starts_with("departure") {
            departure_product *= my_ticket[*field_index];
        }
    }
    departure_product
}

fn main() {
    let input = read_input("input.txt");
    if let Err(e) = &input {
        eprintln!("Error occured: {}", e);
        std::process::exit(1);
    }

    let input = input.unwrap();

    let mut line_index = 0;
    let mut fields = parse_fields(&input, &mut line_index);

    let field_count = fields.len();

    // initialize the fields' possibles indexes (hashset used to find the valid field order)
    fields
        .iter_mut()
        .for_each(|f| f.init_possibles_indexes(field_count));

    let my_ticket = parse_ticket(input[line_index].as_str());

    line_index += 3; // go to nearby tickets

    let err_rate = analyze_nearby_tickets(&mut fields, &input, &mut line_index);
    println!("Part1: {}", err_rate);

    let field_order = deduce_field_order(&mut fields);

    println!(
        "Part2: {}",
        calculate_departure_product(my_ticket, &field_order)
    );
}
