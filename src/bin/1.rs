use std::io;

extern crate aoc2020;
use aoc2020::utils;

fn read_input_numbers() -> Result<Vec<i32>, io::Error> {
    let lines = utils::read_lines("./input_data/1.txt")?;

    Ok(lines
        .filter_map(Result::ok)
        .map(|line| line.parse::<i32>().unwrap())
        .collect())
}

const TARGET_SUM: i32 = 2020;

fn find_two(sorted_numbers: &[i32], target_sum: i32) -> Option<i32> {
    let midpoint_index = sorted_numbers.iter().position(|n| n > &(target_sum / 2)).unwrap();
    let lower_half = &sorted_numbers[..midpoint_index];
    let upper_half = &sorted_numbers[midpoint_index..];

    for n in lower_half {
        let match_index = upper_half.binary_search(&(target_sum - n));
        if match_index.is_ok() {
            return Some(upper_half[match_index.unwrap()] * n);
        }
    }

    None
}

fn find_three(numbers: &[i32]) -> Option<i32> {
    for index in 0..(numbers.len() - 1) {
        let value = numbers[index];
        let remainder = TARGET_SUM - value;
        let result = find_two(&numbers[(index + 1)..], remainder);
        if result.is_some() {
            return Some(value * result.unwrap());
        }
    }

    None
}

fn main() {
    let mut numbers = read_input_numbers().unwrap();
    numbers.sort_unstable();

    let find_two_solution = find_two(&*numbers, TARGET_SUM);
    println!("Find two solution: {}", find_two_solution.unwrap());

    let find_three_solution = find_three(&*numbers);
    println!("Find three solution: {}", find_three_solution.unwrap());
}
