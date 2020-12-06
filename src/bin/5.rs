extern crate aoc2020;

use aoc2020::utils;
use std::cmp::max;

fn char_to_value(c: char, up_char: char, down_char: char) -> usize {
    if c == up_char {
        return 1;
    } else if c == down_char {
        return 0;
    }

    panic!();
}

fn decode_bsp_string(s: &str, up_char: char, down_char: char) -> usize {
    let initial_value = char_to_value(s.chars().nth(0).unwrap(), up_char, down_char);
    s[1..].chars().fold(initial_value, |acc, c| (acc * 2) + char_to_value(c, up_char, down_char))
}

fn get_seat_id(s: &str) -> usize {
    // BFFFBBF RRR
    // B = 1
    // F = 0
    // R = 1
    // L = 0

    let row = decode_bsp_string(&s[..7], 'B', 'F');
    let column = decode_bsp_string(&s[7..], 'R', 'L');

    (row * 8) + column
}

fn main() {
    let lines = utils::read_lines("./input_data/5.txt").unwrap()
        .map(|l| l.unwrap());

   let mut seats_taken = [false; 1024];

    let highest = lines.fold(0, |highest: u32, line| -> u32 {
        let seat_id = get_seat_id(&line);
        seats_taken[seat_id] = true;

        max(highest, seat_id as u32)
    });

    println!("Part 1 highest value: {}", highest);

    let first_taken_seat = seats_taken.iter().position(|s| *s).unwrap();
    let first_empty_seat = seats_taken[first_taken_seat..].iter().position(|s| !*s).unwrap();

    println!("Part 2 seat id: {}", first_empty_seat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(decode_bsp_string("BFFFBBF", 'B', 'F'), 0b1000110);
        assert_eq!(decode_bsp_string("RRR", 'R', 'L'), 0b111);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
    }
}