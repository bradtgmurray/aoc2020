extern crate aoc2020;

use aoc2020::utils;

type Input = (usize, usize, char, String);

fn is_password_valid_part1(min: usize, max: usize, required_char: char, password: &str) -> bool {
    let num_chars = password.chars().filter(|c| *c == required_char).count();
    num_chars >= min && num_chars <= max
}

fn is_password_valid_part2(min: usize, max: usize, required_char: char, password: &str) -> bool {
    let first_index_match = password.chars().nth(min).unwrap() == required_char;
    let second_index_match = password.chars().nth(max).unwrap() == required_char;
    first_index_match && !second_index_match || !first_index_match && second_index_match
}

fn parse_line(line: &String) -> Input {
    let parts: Vec<&str> = line.split(":").collect();
    let spec_parts: Vec<&str> = parts[0].split(" ").collect();
    let range_parts: Vec<usize> =
        spec_parts[0].split("-")
            .map(|n| n.parse::<usize>().unwrap()).collect();

    (range_parts[0], range_parts[1], spec_parts[1].chars().nth(0).unwrap(), parts[1].to_string())
}

fn main() {
    let lines = utils::read_lines("./input_data/2.txt");

    let inputs: Vec<Input> = lines.iter().map(|line| parse_line(line)).collect();

    let failing_passwords_part1 = inputs.iter().filter(|input| -> bool {
        let (min, max, required_char, password) = input;
        is_password_valid_part1(*min, *max, *required_char, password)
    });

    println!("Failing Passwords (part 1): {}", failing_passwords_part1.count());

    let failing_passwords_part2 = inputs.iter().filter(|input| -> bool {
        let (min, max, required_char, password) = input;
        is_password_valid_part2(*min, *max, *required_char, password)
    });

    println!("Failing Passwords (part 2): {}", failing_passwords_part2.count());
}
