extern crate aoc2020;

use aoc2020::utils;
use itertools::Itertools;

fn is_sum_in_slice(sum: usize, slice: &[usize]) -> bool {
    slice.iter().combinations(2).find(|pair| pair[0] + pair[1] == sum).is_some()
}

fn find_invalid_index(numbers: &Vec<usize>) -> usize {
    for i in 25..numbers.len() {
        if !is_sum_in_slice(numbers[i], &numbers[i - 25..i]) {
            return i;
        }
    }

    panic!();
}

fn find_run_with_sum(sum: usize, slice: &[usize]) -> &[usize] {
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            let run = &slice[i..j];
            if run.iter().sum::<usize>() == sum {
                return run;
            }
        }
    }

    panic!();
}

fn main() {
    let numbers: Vec<usize> = utils::read_lines("./input_data/9.txt")
        .iter().map(|l| l.parse::<usize>().unwrap()).collect();

    let invalid_index = find_invalid_index(&numbers);
    println!("Part 1: {}", numbers[invalid_index]);

    let run = find_run_with_sum(numbers[invalid_index], &numbers[0..invalid_index]);
    println!("run: {:?}", run);

    println!("Part 2: {}", run.iter().min().unwrap() + run.iter().max().unwrap())
}
