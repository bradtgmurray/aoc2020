extern crate aoc2020;

// use aoc2020::utils;
use std::collections::HashMap;

fn part1(data: Vec<usize>, num_rounds: usize) -> usize {
    let mut spoken_numbers_to_round: HashMap<usize, usize> = HashMap::new();

    data.iter().enumerate()
        .for_each(|(round, number)| {
            spoken_numbers_to_round.insert(number.clone(), round);
        });

    let mut prev_number = 0;
    let mut current_number = 0;

    for round in data.len()..num_rounds {
        let mut next_number = 0;

        let last_time_spoken = spoken_numbers_to_round.get(&current_number);
        if last_time_spoken.is_some() {
            next_number = round - last_time_spoken.unwrap()
        }

        spoken_numbers_to_round.insert(current_number, round);

        prev_number = current_number;
        current_number = next_number;
    }

    println!("Hash map size: {}", spoken_numbers_to_round.len());

    prev_number
}

fn main() {
    let input = vec![1, 12, 0, 20, 8, 16];
    println!("Part 1: {}", part1(input.clone(), 2020));
    println!("Part 2: {}", part1(input.clone(), 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exmple() {
        assert_eq!(part1(vec![0, 3, 6], 4), 0);
        assert_eq!(part1(vec![0, 3, 6], 5), 3);
        assert_eq!(part1(vec![0, 3, 6], 6), 3);
        assert_eq!(part1(vec![0, 3, 6], 7), 1);
        assert_eq!(part1(vec![0, 3, 6], 8), 0);
        assert_eq!(part1(vec![0, 3, 6], 9), 4);
        assert_eq!(part1(vec![0, 3, 6], 10), 0);
        assert_eq!(part1(vec![0, 3, 6], 2020), 436);
    }
}