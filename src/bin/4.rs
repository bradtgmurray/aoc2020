extern crate aoc2020;

use aoc2020::utils;
use std::collections::{HashSet};

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

type Validator = fn(&str) -> bool;

fn validate_number(s: &str, min: i32, max: i32) -> bool {
    s.parse::<i32>().and_then(|v| Ok(v >= min && v <= max)).unwrap_or(false)
}

fn validate_height(s: &str) -> bool {
    let suffix = &s[(s.len() - 2)..];
    let value = &s[..(s.len() - 2)];
    match suffix {
        "cm" => validate_number(value, 150, 193),
        "in" => validate_number(value, 59, 76),
        _ => false
    }
}

fn validate_colour(s: &str) -> bool {
    s.len() == 7 && s.chars().nth(0).unwrap() == '#' && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

const REQUIRED_FIELDS: [(&str, Validator); 7] =
    [
        ("byr", |s| validate_number(s, 1920, 2002)),
        ("iyr", |s| validate_number(s, 2010, 2020)),
        ("eyr", |s| validate_number(s, 2020, 2030)),
        ("hgt", |s| validate_height(s)),
        ("hcl", |s| validate_colour(s)),
        ("ecl", |s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)),
        ("pid", |s| s.len() == 9 && validate_number(s, 0, 999999999))
    ];

fn validate_passport(passport: &String) -> bool {
    let mut found_fields: HashSet<&str> = HashSet::new();

    let tokens = passport.trim().split(" ");
    tokens.for_each(|t| {
        let field = t.split(":").next().unwrap();
        found_fields.insert(field);
    });

    REQUIRED_FIELDS.iter().all(|(f, _)| found_fields.contains(f))
}

fn validate_passport_part2(passport: &String) -> bool {
    let mut found_fields: HashSet<&str> = HashSet::new();

    let tokens = passport.trim().split(" ");
    tokens.for_each(|t| {
        let mut parts = t.split(":");
        let field_name = parts.next().unwrap();

        let validator = REQUIRED_FIELDS.iter().find(|(k, _)| *k == field_name);
        if validator.is_some() {
            let field_value = parts.next().unwrap();
            let validator_fn = (*(validator.unwrap())).1;
            if validator_fn(field_value) {
                found_fields.insert(field_name);
            }
        }
    });

    REQUIRED_FIELDS.iter().all(|(f, _)| found_fields.contains(f))
}

fn main() {
    let mut valid_passport_count_part1 = 0;
    let mut valid_passport_count_part2 = 0;

    utils::read_groups("./input_data/4.txt", |s| {
        if validate_passport(s) {
            valid_passport_count_part1 += 1;
        }
        if validate_passport_part2(s) {
            valid_passport_count_part2 += 1;
        }
    });

    println!("Part 1 Valid Passports Count: {}", valid_passport_count_part1);
    println!("Part 2 Valid Passports Count: {}", valid_passport_count_part2);
}

// Part 1 Valid Passports Count: 196
// Part 2 Valid Passports Count: 114
