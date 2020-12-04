extern crate aoc2020;

use aoc2020::utils;
use std::collections::{HashSet};

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID) (OPTIONAL)
const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn validate_passport(passport: &String) -> bool {
    let mut found_fields: HashSet<&str> = HashSet::new();

    let tokens = passport.split(" ");
    tokens.for_each(|t| {
        let field = t.split(":").next().unwrap();
        found_fields.insert(field);
    });

    REQUIRED_FIELDS.iter().all(|f| found_fields.contains(f))
}

fn main() {
    let lines = utils::read_lines("./input_data/4.txt").unwrap()
        .map(|l| l.unwrap());

    let mut passport_builder: String = String::new();

    let mut valid_passport_count = 0;

    for line in lines {
        if line.is_empty() {
            if validate_passport(&passport_builder) {
                valid_passport_count += 1;
            }
            passport_builder.clear();
        } else {
            passport_builder.push(' ');
            passport_builder.push_str(line.as_str());
        }
    }

    // Clear out the last one
    if validate_passport(&passport_builder) {
        valid_passport_count += 1;
    }

    println!("Part 1 Valid Passports Count: {}", valid_passport_count)
}