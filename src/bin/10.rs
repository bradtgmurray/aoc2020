extern crate aoc2020;

use aoc2020::utils;
use std::collections::HashMap;

fn find_paths(index: usize, numbers: &Vec<usize>, memoized: &mut Vec<Option<usize>>) -> usize {
    let mut next_index = index + 1;
    let mut paths = 0;

    if index == numbers.len() - 1 {
        // Successful end path found
        return 1;
    }

    if memoized[index].is_some() {
        return memoized[index].unwrap();
    }

    while next_index < numbers.len() {
        if numbers[next_index] <= numbers[index] + 3 {
            paths += find_paths(next_index, numbers, memoized);
        }
        next_index += 1;
    }

    memoized[index] = Some(paths);
    paths
}

fn find_paths_begin(numbers: &Vec<usize>) -> usize {
    let mut memoized: Vec<Option<usize>> = vec![Option::None; numbers.len()];

    let mut paths = 0;

    let mut index = 0;
    while index < numbers.len() && numbers[index] <= 3 {
        paths += find_paths(index, numbers, &mut memoized);
        index += 1;
    }

    paths
}

fn main() {
    let mut numbers: Vec<usize> = utils::read_lines("./input_data/10.txt")
        .iter().map(|l| l.parse::<usize>().unwrap()).collect();

    numbers.sort();

    let mut counts: HashMap<usize, usize> = HashMap::new();

    // Add the initial jump
    counts.insert(numbers[0], 1);

    // Add the final jump
    *counts.entry(3).or_insert(0) += 1;

    for i in 0..(numbers.len() - 1) {
        let diff = numbers[i + 1] - numbers[i];

        *counts.entry(diff).or_insert(0) += 1;
    }

    println!("Part 1: {}", counts.get(&1).unwrap() * counts.get(&3).unwrap());

    println!("Part 2: {}", find_paths_begin(&numbers));
}


#[cfg(test)]
mod tests {
    use super::*;

    fn find_paths_test(numbers: &mut Vec<usize>) -> usize {
        numbers.sort();
        find_paths_begin(numbers)
    }

    #[test]
    fn test_simple() {
        assert_eq!(find_paths_test(&mut vec![1]), 1);
        assert_eq!(find_paths_test(&mut vec![1, 2]), 2);
        // 123, 13, 23, 3
        assert_eq!(find_paths_test(&mut vec![1, 2, 3]), 4);
        // 1234, 124, 134, 14, 234, 24, 34
        assert_eq!(find_paths_test(&mut vec![1, 2, 3, 4]), 7);
    }

    #[test]
    fn test_examples() {
        assert_eq!(find_paths_test(&mut vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4]), 8);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(find_paths_test(&mut vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ]), 19208);
    }
}
