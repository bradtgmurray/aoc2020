extern crate aoc2020;

use aoc2020::utils;

fn parse(line: &String) -> (char, usize) {
    (
        line.chars().nth(0).unwrap(),
        line.chars().skip(1).collect::<String>().parse::<usize>().unwrap()
    )
}

fn rotate_deg(point: (i32, i32), center: (i32, i32), clockwise_degress: i32) -> (i32, i32) {
    let rads = (clockwise_degress as f32).to_radians();
    (
        center.0 + (rads.cos() as i32 * (point.0 - center.0) + rads.sin() as i32 * (point.1 - center.1)),
        center.1 + (-rads.sin() as i32 * (point.0 - center.0) + rads.cos() as i32 * (point.1 - center.1))
    )
}

fn execute_instructions_part1(data: &Vec<String>) -> usize {
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
            'L' => facing = rotate_deg(facing, (0, 0), -(value as i32)),
            'R' => facing = rotate_deg(facing, (0, 0), value as i32),
            'F' => {
                position.0 += facing.0 * value as i32;
                position.1 += facing.1 * value as i32;
            }
            _ => panic!()
        }
    });

    (position.0.abs() + position.1.abs()) as usize
}

fn execute_instructions_part2(data: &Vec<String>) -> usize {
    let mut position: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);

    // Action N means to move north by the given value.
    // Action S means to move south by the given value.
    // Action E means to move east by the given value.
    // Action W means to move west by the given value.
    // Action L means to turn left the given number of degrees.
    // Action R means to turn right the given number of degrees.
    // Action F means to move forward by the given value in the direction the ship is currently facing.

    data.iter().map(|l| parse(l)).for_each(|(op, value)| {
        match op {
            'N' => waypoint.1 += value as i32,
            'S' => waypoint.1 -= value as i32,
            'E' => waypoint.0 += value as i32,
            'W' => waypoint.0 -= value as i32,
            'L' => waypoint = rotate_deg(waypoint, position, -(value as i32)),
            'R' => waypoint = rotate_deg(waypoint, position, value as i32),
            'F' => {
                let movement = (
                    (waypoint.0 - position.0) * value as i32,
                    (waypoint.1 - position.1) * value as i32
                );

                position.0 += movement.0;
                position.1 += movement.1;

                waypoint.0 += movement.0;
                waypoint.1 += movement.1;
            }
            _ => panic!()
        }
    });

    (position.0.abs() + position.1.abs()) as usize
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/12.txt");
    println!("Part 1: {}", execute_instructions_part1(&data));
    println!("Part 2: {}", execute_instructions_part2(&data));
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
        assert_eq!(execute_instructions_part1(&vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].iter().map(|s| s.to_string()).collect()), 25);
    }

    #[test]
    fn test_part2() {
        // F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
        // N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
        // F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
        // R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
        // F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.

        assert_eq!(execute_instructions_part2(&vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].iter().map(|s| s.to_string()).collect()), 286);
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_deg((1, 0), (0, 0), -90), (0, 1));
        assert_eq!(rotate_deg((1, 0), (0, 0), 90), (0, -1));
        assert_eq!(rotate_deg((1, 0), (0, 0), -450), (0, 1));
        assert_eq!(rotate_deg((1, 0), (0, 0), 450), (0, -1));
    }
}