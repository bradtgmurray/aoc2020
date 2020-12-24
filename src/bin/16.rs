extern crate aoc2020;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use aoc2020::utils;
use regex::Regex;
use std::ops::Range;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<Range<usize>>,
}

impl Rule {
    fn includes_value(&self, v: &usize) -> bool {
        self.ranges.iter().any(|r| r.contains(v))
    }
}

type Ticket = Vec<usize>;

fn parse_rule(line: &String) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)\-(\d+) or (\d+)\-(\d+)$").unwrap();
    }

    let cap = RE.captures_iter(line).next().unwrap();

    let name: String = cap[1].to_string();

    let ranges = vec![
        (cap[2].parse::<usize>().unwrap()..(cap[3].parse::<usize>().unwrap()) + 1),
        (cap[4].parse::<usize>().unwrap()..(cap[5].parse::<usize>().unwrap()) + 1)
    ];

    Rule { name, ranges }
}

fn parse_ticket(line: String) -> Ticket {
    line.split(",").map(|v| v.parse::<usize>().unwrap()).collect()
}

fn parse_file(data: &Vec<String>) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut line_iter = data.iter();

    let mut rules: Vec<Rule> = vec![];
    loop {
        let line = line_iter.next().unwrap();

        if line.is_empty() {
            break;
        }

        rules.push(parse_rule(line));
    }

    // Drop the "your ticket:" line
    line_iter.next();

    let ticket = parse_ticket(line_iter.next().unwrap().to_string());

    // Drop the empty line
    line_iter.next();

    // Drop the "nearby tickets:" line
    line_iter.next();

    let nearby_tickets: Vec<Ticket> = line_iter.map(|l| parse_ticket(l.to_string())).collect();

    (rules, ticket, nearby_tickets)
}

fn part1(data: &Vec<String>) -> usize {
    let (rules, _, nearby_tickets) = parse_file(&data);

    let invalid_values = nearby_tickets.iter().map(|t| {
        // Filter down to the invalid values
        t.iter()
            .map(|v| {
                if rules.iter().any(|r| r.includes_value(&v)) {
                    Ok(v)
                } else {
                    Err(v)
                }
            })
            .filter(|v| v.is_err())
            .map(|v| v.unwrap_err())
    }).flatten();

    invalid_values.sum()
}

fn part2(data: &Vec<String>) -> usize {
    let (rules, your_ticket, nearby_tickets) = parse_file(&data);

    let valid_nearby_tickets: Vec<&Ticket> = nearby_tickets.iter().filter(|t| {
        // Ticket is valid if all the fields match any rule
        t.iter().all(|v| rules.iter().any(|r| r.includes_value(&v)))
    }).collect();

    let mut rules_with_possible_matches: Vec<(&Rule, Vec<usize>)> = rules.iter()
        .map(|r| {
            // Find the column index that matches this rule. In order for it to match, all tickets
            // must match the rule at the current index
            let matching_columns: Vec<usize> = (0..your_ticket.len()).filter(|i|
                // Is it the i'th column?
                valid_nearby_tickets.iter().all(|t| r.includes_value(&t[*i]))
            ).collect();

            (r, matching_columns)
        }).collect();

    let mut solved_rules: Vec<String> = vec![];

    // Keep looping until we solve all the departure rules
    while solved_rules.iter().filter(|r| r.starts_with("departure")).count() != 6 {
        let (matched_rule, matched_indexes) = rules_with_possible_matches.iter()
            .filter(|(r, _)| solved_rules.iter().find(|sr| sr == &&r.name).is_none())
            .find(|(_, v)| v.len() == 1).unwrap().clone();

        let matched_index = matched_indexes[0]; // There's only one by definition, we're good

        println!("Found {} {}", matched_rule.name, matched_index);
        solved_rules.push(matched_rule.name.clone());

        for (r, i) in rules_with_possible_matches.iter_mut() {
            if r.name != matched_rule.name {
                i.retain(|v| v != &matched_index)
            }
        }
    }

    rules_with_possible_matches.iter()
        .filter(|(r, _)| r.name.starts_with("departure"))
        .map(|(_, i)| your_ticket[i[0]]).product()
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/16.txt");

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_input: Vec<String> = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12"
        ].iter().map(|s| String::from(s.clone())).collect();

        let (rules, your_ticket, nearby_tickets) = parse_file(&test_input);
        assert_eq!(rules.len(), 3);
        assert_eq!(rules[1].name, "row");
        assert_eq!(rules[1].ranges.len(), 2);
        assert_eq!(rules[1].ranges[0].start, 6);
        assert_eq!(rules[1].ranges[0].end, 12);
        assert_eq!(rules[1].ranges[1].start, 33);
        assert_eq!(rules[1].ranges[1].end, 45);

        assert_eq!(your_ticket, [7, 1, 14]);

        assert_eq!(nearby_tickets.len(), 4);
        assert_eq!(nearby_tickets[0], [7, 3, 47]);
        assert_eq!(nearby_tickets[1], [40, 4, 50]);

        assert_eq!(part1(&test_input), 71);
    }
}