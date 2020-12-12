extern crate aoc2020;

use aoc2020::utils;
use itertools::Itertools;

fn parse(line: &String) -> (char, usize) {
    (
        line.chars().nth(0).unwrap(),
        line.chars().skip(1).collect::<String>().parse::<usize>().unwrap()
    )
}

const CLOCKWISE_FACINGS: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn current_facing_index(facing: (i32, i32)) -> usize {
    CLOCKWISE_FACINGS.iter().find_position(|f| **f == facing).unwrap().0
}

fn rotate_deg(facing: (i32, i32), clockwise_degress: i32) -> (i32, i32) {
    let current_index = current_facing_index(facing);
    let new_index = ((current_index as i32) + (clockwise_degress / 90)).rem_euclid(CLOCKWISE_FACINGS.len() as i32) as usize;
    CLOCKWISE_FACINGS[new_index]
}

fn execute_instructions(data: &Vec<String>) -> usize {
    let mut position: (i32, i32) = (0, 0);
    let mut facing: (i32, i32) = (1, 0);

    // Action N means to move north by the given value.
    // Action S means to move south by the given value.
    // Action E means to move east by the given value.
    // Action W means to move west by the given value.
    // Action L means to turn left the given number of degrees.
    // Action R means to turn right the given number of degrees.
    // Action F means to move forward by the given value in the direction the ship is currently facing.

    data.iter().map(|l| parse(l)).for_each(|(op, value)| {
        match op {
            'N' => position.1 += value as i32,
            'S' => position.1 -= value as i32,
            'E' => position.0 += value as i32,
            'W' => position.0 -= value as i32,
            'L' => facing = rotate_deg(facing, -(value as i32)),
            'R' => facing = rotate_deg(facing, value as i32),
            'F' => {
                position.0 += facing.0 * value as i32;
                position.1 += facing.1 * value as i32;
            }
            _ => panic!()
        }
    });

    (position.0.abs() + position.1.abs()) as usize
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/12.txt");
    println!("Part 1: {}", execute_instructions(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(&String::from("F10"));
        assert_eq!(result.0, 'F');
        assert_eq!(result.1, 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(execute_instructions(&vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].iter().map(|s| s.to_string()).collect()), 25);
    }


    #[test]
    fn test_rotate() {
        assert_eq!(rotate_deg((1, 0), -90), (0, 1));
        assert_eq!(rotate_deg((1, 0), 90), (0, -1));
        assert_eq!(rotate_deg((1, 0), -450), (0, 1));
        assert_eq!(rotate_deg((1, 0), 450), (0, -1));
    }
}