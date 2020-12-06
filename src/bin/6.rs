extern crate aoc2020;

use aoc2020::utils;
use std::collections::HashSet;

fn str_to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn count_unanimous_answers(s: &str) -> usize {
    let mut groups = s.split_ascii_whitespace();
    let first_group = groups.next().unwrap();

    // Seed the set with all th answers from the first group
    let mut chars_seen: HashSet<char> = str_to_set(first_group);

    // For each remaining group, remove anything from the set that's not in the group
    groups.for_each(|g| {
        chars_seen = chars_seen.intersection(&str_to_set(g)).map(|c| *c).collect();
    });

    chars_seen.len()
}

fn main() {
    let mut answers_seen = 0;

    utils::read_groups("./input_data/6.txt", |s| {
        let mut chars_seen: HashSet<char> = HashSet::new();
        s.chars().for_each(|c| if c.is_alphabetic() { chars_seen.insert(c); });
        answers_seen += chars_seen.len();
    });

    println!("Part 1: {}", answers_seen);

    let mut unanimous_answers_seen = 0;

    utils::read_groups("./input_data/6.txt", |s| {
        unanimous_answers_seen += count_unanimous_answers(s)
    });

    println!("Part 2: {}", unanimous_answers_seen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(count_unanimous_answers("abc ab ab abc"), 2);
    }
}
