extern crate aoc2020;

use aoc2020::utils;
use std::iter::{FromIterator};

struct SeatMap {
    width: i32,
    height: i32,
    data: Vec<char>,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

impl SeatMap {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.data[(x + (y * self.width)) as usize])
        } else {
            None
        }
    }

    fn get_surrounding(&self, x: i32, y: i32) -> Vec<char> {
        DIRECTIONS.iter()
            .map(|(delta_x, delta_y)| self.get(x + delta_x, y + delta_y))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    fn get_visible_surrounding(&self, x: i32, y: i32) -> Vec<char> {
        DIRECTIONS.iter()
            .map(|(delta_x, delta_y)| -> Option<char> {
                let mut i = 1;
                loop {
                    let c = self.get(x + (delta_x * i), y + (delta_y * i));
                    if c.is_none() {
                        return None;
                    } else if c.unwrap() == 'L' || c.unwrap() == '#' {
                        return c;
                    }
                    i += 1;
                }
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Printing map");
        self.data
            .chunks(self.width as usize)
            .for_each(|s| println!("{}", s.iter().collect::<String>()));
        println!("---");
    }
}

fn gen_next_map_part1(seat_map: &SeatMap) -> Option<SeatMap> {
    let next_map: Vec<char> = seat_map.data.iter().enumerate().map(|(i, c)| {
        let x = i % seat_map.width as usize;
        let y = i / seat_map.width as usize;
        let surrounding = seat_map.get_surrounding(x as i32, y as i32);

        if *c == 'L' {
            // empty seats become occupied if none of the seats around are occupied
            if !surrounding.iter().any(|s| *s == '#') {
                '#'
            } else {
                'L'
            }
        } else if *c == '#' {
            // occupied seats become empty if 4 or more of the seats around are occupied
            if surrounding.iter().filter(|s| **s == '#').count() >= 4 {
                'L'
            } else {
                '#'
            }
        } else {
            // floor spot
            '.'
        }
    }).collect();

    if seat_map.data != next_map {
        Some(SeatMap {
            data: next_map,
            width: seat_map.width,
            height: seat_map.height,
        })
    } else {
        None
    }
}

fn run_part1(data: &Vec<String>) -> usize {
    let width: i32 = data.first().unwrap().len() as i32;
    let height: i32 = data.len() as i32;

    let mut map: SeatMap = SeatMap {
        data: Vec::from_iter(data.iter().map(|s| s.chars()).flatten()),
        width,
        height,
    };

    loop {
        let next_map = gen_next_map_part1(&map);
        if next_map.is_none() {
            // Done! Count the seats
            return map.data.iter().filter(|c| **c == '#').count();
        }

        map = next_map.unwrap();
    };
}

fn gen_next_map_part2(seat_map: &SeatMap) -> Option<SeatMap> {
    let next_map: Vec<char> = seat_map.data.iter().enumerate().map(|(i, c)| {
        let x = i % seat_map.width as usize;
        let y = i / seat_map.width as usize;
        let surrounding = seat_map.get_visible_surrounding(x as i32, y as i32);

        if *c == 'L' {
            // empty seats become occupied if none of the seats around are occupied
            if !surrounding.iter().any(|s| *s == '#') {
                '#'
            } else {
                'L'
            }
        } else if *c == '#' {
            // occupied seats become empty if 5 or more of the seats around are occupied
            if surrounding.iter().filter(|s| **s == '#').count() >= 5 {
                'L'
            } else {
                '#'
            }
        } else {
            // floor spot
            '.'
        }
    }).collect();

    if seat_map.data != next_map {
        Some(SeatMap {
            data: next_map,
            width: seat_map.width,
            height: seat_map.height,
        })
    } else {
        None
    }
}

fn run_part2(data: &Vec<String>) -> usize {
    let width: i32 = data.first().unwrap().len() as i32;
    let height: i32 = data.len() as i32;

    let mut map: SeatMap = SeatMap {
        data: Vec::from_iter(data.iter().map(|s| s.chars()).flatten()),
        width,
        height,
    };

    loop {
        let next_map = gen_next_map_part2(&map);
        if next_map.is_none() {
            // Done! Count the seats
            return map.data.iter().filter(|c| **c == '#').count();
        }

        map = next_map.unwrap();
    };
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/11.txt");
    println!("Part 1: {}", run_part1(&data));
    println!("Part 2: {}", run_part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(&[
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL"
        ].iter().map(|s| s.to_string()).collect()), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2(&[
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL"
        ].iter().map(|s| s.to_string()).collect()), 26);
    }
}