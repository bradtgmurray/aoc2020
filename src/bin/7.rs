extern crate aoc2020;

use aoc2020::utils;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
#[derive(PartialEq)]
struct BagGroup {
    name: String,
    count: usize,
}

fn parse_line(line: &str) -> (&str, Vec<BagGroup>) {
    let mut parts = line.split(" bags contain ");
    let container = parts.next().unwrap();

    let contained_str = parts.next().unwrap();
    if contained_str.starts_with("no ") {
        return (container, Vec::new());
    }

    let contained_str_parts = contained_str.split(", ");

    // Each part now has a count, a bag name (two words), and the word bag
    let contained: Vec<BagGroup> = contained_str_parts.map(|bag_str| {
        let mut words = bag_str.split_ascii_whitespace();
        let bag_count = words.next().unwrap().parse::<usize>().unwrap();
        let bag_words: Vec<&str> = words.take(2).collect();

        let bag = bag_words.join(" ");

        BagGroup { name: bag, count: bag_count }
    }).collect();

    (container, contained)
}

fn part1() {
    // Map of bags to which bags are allowed to contain them
    let mut contains_map: HashMap<String, HashSet<String>> = HashMap::new();

    utils::read_lines("./input_data/7.txt").iter()
        .map(|l| parse_line(l))
        .for_each(|rule| {
            let (container, contained) = rule;
            contained.iter().for_each(|c| {
                contains_map.entry(c.name.clone()).or_insert(HashSet::new()).insert(String::from(container));
            });
        });

    let mut shiny_gold_containers: HashSet<String> = HashSet::new();

    fn add_containers(b: &str, map: &HashMap<String, HashSet<String>>, set: &mut HashSet<String>) {
        let containers = map.get(b);
        if containers.is_some() {
            containers.unwrap().iter().for_each(|c| {
                set.insert(String::from(c));
                add_containers(c, map, set);
            })
        }
    }

    add_containers("shiny gold", &contains_map, &mut shiny_gold_containers);

    println!("Part 1: {} containers", shiny_gold_containers.len())
}

fn part2() {
    let mut rule_map: HashMap<String, Vec<BagGroup>> = HashMap::new();

    utils::read_lines("./input_data/7.txt").iter()
        .map(|l| parse_line(l))
        .for_each(|rule| {
            rule_map.insert(String::from(rule.0), rule.1);
        });

    fn count_contained_bags(b: &str, map: &HashMap<String, Vec<BagGroup>>) -> usize {
        let contained = map.get(b);
        if contained.is_none() {
            0
        } else {
            contained.unwrap().iter().map(|c| {
                c.count * (1 + count_contained_bags(c.name.as_str(), map))
            }).sum()
        }
    }

    let count = count_contained_bags("shiny gold", &rule_map);
    println!("Part 2: {} bags", count);
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(parse_line("bright lime bags contain no other bags."),
                   ("bright lime", Vec::from([])));
        assert_eq!(parse_line("shiny aqua bags contain 1 dark white bag."),
                   ("shiny aqua", Vec::from([BagGroup { name: String::from("dark white"), count: 1 }])));
        assert_eq!(parse_line("muted blue bags contain 1 vibrant lavender bag, 4 dotted silver bags, 2 dim indigo bags."),
                   ("muted blue", Vec::from(
                       [
                           BagGroup { name: String::from("vibrant lavender"), count: 1 },
                           BagGroup { name: String::from("dotted silver"), count: 4 },
                           BagGroup { name: String::from("dim indigo"), count: 2 }
                       ])));
    }
}
