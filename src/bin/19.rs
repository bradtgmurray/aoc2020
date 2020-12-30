use aoc2020::utils;
use std::collections::HashMap;

type RuleMap = HashMap<usize, Rule>;

// A rule either has a value or a list of child rules
#[derive(Debug)]
struct Rule {
    value: Option<char>,
    child_rules: Vec<ChildRule>,
}

#[derive(Debug)]
struct ChildRule {
    rule_sequence: Vec<usize>
}

fn rule_is_match(rule_map: &RuleMap, s: &str, sequence: &[usize]) -> bool {
    // it's a match if the first rule in the sequence is a match as well as the rest

    // If we have no more characters to parse, we're done only if the sequence is empty.
    if s.is_empty() || sequence.is_empty() {
        return s.is_empty() && sequence.is_empty();
    }

    let first_rule = rule_map.get(&sequence[0]).unwrap();
    if first_rule.value.is_some() {
        return if s.chars().nth(0).unwrap() == first_rule.value.unwrap() {
            // Match! Pop this rule and see if the rest matches
            rule_is_match(rule_map, &s[1..], &sequence[1..])
        } else {
            false
        };
    }

    // Else we have child rules. We match if either sequence of children match
    first_rule.child_rules.iter().any(|cr| {
        let mut new_sequence = cr.rule_sequence.clone();
        new_sequence.extend_from_slice(&sequence[1..]);
        rule_is_match(rule_map, s, &new_sequence)
    })
}

fn parse(data: &Vec<String>) -> (RuleMap, &[String]) {
    let mut rule_map: RuleMap = HashMap::new();

    for line in data {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");

        let rule_name = parts.next().unwrap();
        let rule_string = parts.next().unwrap();

        let rule: Rule;
        if rule_string.starts_with("\"") {
            rule = Rule { value: Some(rule_string.chars().nth(1).unwrap()), child_rules: vec![] }
        } else {
            rule = Rule {
                value: None,
                child_rules: rule_string.split(" | ").map(|child_rule_string|
                    ChildRule { rule_sequence: child_rule_string.split(" ").map(|i| i.parse::<usize>().unwrap()).collect() }
                ).collect(),
            }
        }

        rule_map.insert(rule_name.parse::<usize>().unwrap(), rule);
    }

    let messages = &data[(rule_map.len() + 1)..];

    (rule_map, messages)
}

fn part1(data: &Vec<String>) -> usize {
    let (rule_map, messages) = parse(&data);
    let rule_zero = rule_map.get(&0).unwrap();
    messages.iter().filter(|m| {
        rule_is_match(&rule_map, m, &rule_zero.child_rules[0].rule_sequence)
    }).count()
}

fn apply_part2_changes(rule_map: &mut RuleMap) {
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rule_map.insert(8, Rule {
        value: None,
        child_rules: vec![
            ChildRule { rule_sequence: vec![42] },
            ChildRule { rule_sequence: vec![42, 8] },
        ],
    });

    rule_map.insert(11, Rule {
        value: None,
        child_rules: vec![
            ChildRule { rule_sequence: vec![42, 31] },
            ChildRule { rule_sequence: vec![42, 11, 31] },
        ],
    });
}

fn part2(data: &Vec<String>) -> usize {
    let (mut rule_map, messages) = parse(&data);
    apply_part2_changes(&mut rule_map);

    let rule_zero = rule_map.get(&0).unwrap();

    messages.iter().filter(|m| {
        rule_is_match(&rule_map, m, &rule_zero.child_rules[0].rule_sequence)
    }).count()
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/19.txt");

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rule() {
        let mut rm: RuleMap = HashMap::new();
        rm.insert(1, Rule { value: Some('a'), child_rules: vec![] });
        rm.insert(2, Rule { value: Some('b'), child_rules: vec![] });
        rm.insert(3, Rule {
            value: None,
            child_rules: vec![ChildRule { rule_sequence: vec![1, 2] }],
        });

        assert_eq!(rule_is_match(&rm, "a", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "b", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "ab", &vec![3]), true);
        assert_eq!(rule_is_match(&rm, "ba", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "abc", &vec![3]), false);
    }

    #[test]
    fn test_loop_rule() {
        let mut rm: RuleMap = HashMap::new();
        rm.insert(1, Rule { value: Some('a'), child_rules: vec![] });
        rm.insert(2, Rule { value: Some('b'), child_rules: vec![] });
        rm.insert(3, Rule {
            value: None,
            child_rules: vec![ChildRule { rule_sequence: vec![1, 2] }, ChildRule { rule_sequence: vec![1, 3] }],
        });

        rm.insert(4, Rule {
            value: None,
            child_rules: vec![ChildRule { rule_sequence: vec![1, 2] }, ChildRule { rule_sequence: vec![1, 4, 2] }],
        });

        assert_eq!(rule_is_match(&rm, "a", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "b", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "ab", &vec![3]), true);
        assert_eq!(rule_is_match(&rm, "ba", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "abc", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "aab", &vec![3]), true);
        assert_eq!(rule_is_match(&rm, "abab", &vec![3]), false);
        assert_eq!(rule_is_match(&rm, "aaab", &vec![3]), true);

        assert_eq!(rule_is_match(&rm, "aabb", &vec![4]), true);
        assert_eq!(rule_is_match(&rm, "aaabbb", &vec![4]), true);
    }


    #[test]
    fn test_example_part1() {
        let data: Vec<String> = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb"
        ].iter().map(|s| String::from(*s)).collect();

        let (rule_map, messages) = parse(&data);

        assert_eq!(rule_map.len(), 6);
        assert_eq!(rule_map.get(&0).unwrap().value, None);
        assert_eq!(rule_map.get(&0).unwrap().child_rules.len(), 1);
        assert_eq!(rule_map.get(&0).unwrap().child_rules[0].rule_sequence, [4, 1, 5]);
        assert_eq!(rule_map.get(&5).unwrap().value, Some('b'));

        assert_eq!(messages.len(), 5);
    }

    #[test]
    fn test_complex_example_part2() {
        let data: Vec<String> = vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        ].iter().map(|s| String::from(*s)).collect();

        let (mut rule_map, _) = parse(&data);

        // assert_eq!(rule_map.get(&0).unwrap().is_match("aaaaabbaabaaaaababaa", &rule_map), false);

        apply_part2_changes(&mut rule_map);

        assert_eq!(rule_is_match(&rule_map, "aaaaabbaabaaaaababaa", &rule_map.get(&0).unwrap().child_rules[0].rule_sequence), true);

        //assert_eq!(part1(&data), 3);
        //assert_eq!(part2(&data), 12);
    }
}