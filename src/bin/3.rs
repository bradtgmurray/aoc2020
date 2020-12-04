extern crate aoc2020;

use aoc2020::utils;

fn count_trees(right: usize, down: usize, lines: &Vec<String>) -> usize {
    let mut x_index = 0;
    lines.iter().step_by(down).filter(|line| -> bool {
        let hit = line.chars().nth(x_index).unwrap() == '#';
        x_index = (x_index + right) % line.chars().count();

        hit
    }).count()
}

fn main() {
    let lines: Vec<String> = utils::read_lines("./input_data/3.txt").unwrap()
        .map(|l| l.unwrap())
        .collect();

    let part1_trees_hit = count_trees(3, 1, &lines);

    println!("Part1: Hit {} trees", part1_trees_hit);

    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let part2_trees_hit = count_trees(1, 1, &lines) *
        count_trees(3, 1, &lines) *
        count_trees(5, 1, &lines) *
        count_trees(7, 1, &lines) *
        count_trees(1, 2, &lines);

    println!("Part2: Hit {} trees", part2_trees_hit);
}
