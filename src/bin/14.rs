extern crate aoc2020;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use aoc2020::utils;
use regex::Regex;
use std::collections::HashMap;

fn parse_mask(line: &String) -> (u64, u64, u64) {
    let mut one_mask: u64 = 0;
    let mut zero_mask: u64 = 0;
    let mut float_mask: u64 = 0;

    // skip "mask = "
    let bits = line.chars().skip(7);
    bits.for_each(|b| {
        match b {
            '1' => one_mask = one_mask | 0x1,
            '0' => zero_mask = zero_mask | 0x1,
            'X' => float_mask = float_mask | 0x1,
            _ => panic!(),
        };

        one_mask = one_mask << 1;
        zero_mask = zero_mask << 1;
        float_mask = float_mask << 1;
    });

    // We over shift in our loop, shift it back down
    (one_mask >> 1, zero_mask >> 1, float_mask >> 1)
}


fn parse_mem(line: &String) -> (u64, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let cap = RE.captures_iter(line).next().unwrap();

    (cap[1].parse::<u64>().unwrap(), cap[2].parse::<u64>().unwrap())
}

fn part1(data: &Vec<String>) -> u64 {
    let mut one_mask: u64 = 0;
    let mut zero_mask: u64 = 0;
    let mut memory_map: HashMap<u64, u64> = HashMap::new();

    data.iter().for_each(|line| {
        if line.starts_with("mask") {
            let new_mask = parse_mask(line);
            one_mask = new_mask.0;
            zero_mask = new_mask.1;
        } else {
            let (addr, mut value) = parse_mem(line);
            value = value & !zero_mask;
            value = value | one_mask;
            memory_map.insert(addr, value);
        }
    });

    memory_map.values().sum()
}

fn update_address_with_float(memory_map: &mut HashMap<u64, u64>, float_mask: u64, addr: u64, value: u64) {
    if float_mask == 0 {
        memory_map.insert(addr, value);
    } else {
        for i in 0..36 {
            if float_mask & (1 << i) != 0 {
                // Remove the float bit
                let new_float_mask = float_mask & !(1 << i);
                // Recursively assign both
                update_address_with_float(memory_map, new_float_mask, addr | (1 << i), value);
                update_address_with_float(memory_map, new_float_mask, addr & (!(1 << i)), value);

                break;
            }
        }
    }
}

fn part2(data: &Vec<String>) -> u64 {
    let mut one_mask: u64 = 0;
    let mut float_mask: u64 = 0;
    let mut memory_map: HashMap<u64, u64> = HashMap::new();

    data.iter().for_each(|line| {
        if line.starts_with("mask") {
            let new_mask = parse_mask(line);
            one_mask = new_mask.0;
            float_mask = new_mask.2;
        } else {
            let (mut addr, value) = parse_mem(line);
            addr = addr | one_mask;
            update_address_with_float(&mut memory_map, float_mask, addr, value);
        }
    });

    memory_map.values().sum()
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/14.txt");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mask() {
        let (one, zero, float) = parse_mask(&String::from("mask = 1X0X0"));
        assert_eq!(one, 0b10000);
        assert_eq!(zero, 0b00101);
        assert_eq!(float, 0b01010);
    }

    #[test]
    fn test_parse_mem() {
        let (addr, value) = parse_mem(&String::from("mem[41026] = 409998"));
        assert_eq!(addr, 41026);
        assert_eq!(value, 409998);
    }

    #[test]
    fn test_part2() {
        let data = vec![
            String::from("mask = 000000000000000000000000000000X1001X"),
            String::from("mem[42] = 100"),
            String::from("mask = 00000000000000000000000000000000X0XX"),
            String::from("mem[26] = 1")
        ];
        assert_eq!(part2(&data), 208)
    }
}