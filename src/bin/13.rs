extern crate aoc2020;

use aoc2020::utils;
use itertools::Itertools;

fn part1(departure_time: usize, raw_bus_id_list: &Vec<Option<usize>>) -> usize {
    let bus_id_list: Vec<usize> = raw_bus_id_list.iter()
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .collect();
    let missed_by_times: Vec<usize> = bus_id_list.iter().map(|b| departure_time % *b).collect();
    let wait_times: Vec<usize> = bus_id_list.iter().zip(missed_by_times).map(|(b, m)| b - m).collect();

    let min_index = wait_times.iter().position_min().unwrap();

    bus_id_list[min_index] * wait_times[min_index]
}

fn part2(departure_time: usize, raw_bus_id_list: &Vec<Option<usize>>) -> usize {
    let largest_number_index = raw_bus_id_list.iter().position_max().unwrap();
    let largest_number = raw_bus_id_list[largest_number_index].unwrap();

    let mut multiple = (departure_time / largest_number) + 1;
    loop {
        let start = multiple * largest_number - largest_number_index;
        let result = raw_bus_id_list.iter().enumerate().all(|(i, b)| {
            if b.is_none() {
                true
            } else {
                (start + i) % b.unwrap() == 0
            }
        });

        if result {
            return start;
        }

        multiple += 1;
    }
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/13.txt");

    let departure_time = data[0].parse::<usize>().unwrap();
    let bus_id_list: Vec<Option<usize>> = data[1]
        .split(",")
        .map(|b| {
            if b == "x" {
                None
            } else {
                Some(b.parse::<usize>().unwrap())
            }
        })
        .collect();

    println!("Part 1: {}", part1(departure_time, &bus_id_list));
    println!("Part 2: {}", part2(departure_time, &bus_id_list));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_simple() {
        let bus_list = vec![Some(2),Some(3)];
        assert_eq!(part2(0, &bus_list), 2)
    }


    #[test]
    fn test_part2_simple2() {
        let bus_list = vec![Some(4),Some(3)];
        assert_eq!(part2(0, &bus_list), 8)
    }

    #[test]
    fn test_part2_ex1() {
        let bus_list = vec![Some(17),None,Some(13),Some(19)];
        assert_eq!(part2(0, &bus_list), 3417)
    }

    #[test]
    fn test_part2_ex4() {
        let bus_list = vec![Some(67),Some(7), None,Some(59),Some(61)];
        assert_eq!(part2(0, &bus_list), 1261476)
    }
}