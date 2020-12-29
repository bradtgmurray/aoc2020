extern crate aoc2020;

use std::collections::HashMap;

type Vector3 = (i32, i32, i32);
type State3 = HashMap<Vector3, bool>;

fn get_neighbours(v: Vector3) -> Vec<Vector3> {
    let mut neighbours: Vec<Vector3> = Vec::with_capacity(26);

    for x in (v.0 - 1)..(v.0 + 2) {
        for y in (v.1 - 1)..(v.1 + 2) {
            for z in (v.2 - 1)..(v.2 + 2) {
                if x != v.0 || y != v.1 || z != v.2 {
                    neighbours.push((x, y, z));
                }
            }
        }
    }

    neighbours
}

fn count_active_neighbours(state: &State3, v: Vector3) -> usize {
    get_neighbours(v).iter().filter(|nv| *state.get(nv).unwrap_or(&false)).count()
}

fn run_iteration(state: State3) -> State3 {
    let mut next_state: State3 = State3::new();

    let min_x = state.keys().map(|v| v.0).min().unwrap();
    let max_x = state.keys().map(|v| v.0).max().unwrap();
    let min_y = state.keys().map(|v| v.1).min().unwrap();
    let max_y = state.keys().map(|v| v.1).max().unwrap();
    let min_z = state.keys().map(|v| v.2).min().unwrap();
    let max_z = state.keys().map(|v| v.2).max().unwrap();

    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                let v = (x, y, z);
                let active_neighbours = count_active_neighbours(&state, v);

                let cur_state: bool = *state.get(&v).unwrap_or(&false);
                if cur_state {
                    if active_neighbours == 2 || active_neighbours == 3 {
                        next_state.insert(v, true);
                    } else {
                        next_state.insert(v, false);
                    }
                } else {
                    if active_neighbours == 3 {
                        next_state.insert(v, true);
                    } else {
                        next_state.insert(v, false);
                    }
                }
            }
        }
    }

    next_state
}

fn part1(data: &Vec<String>) -> usize {
    let mut state: State3 = State3::new();

    data.iter().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                state.insert((x as i32, y as i32, 0), true);
            }
        });
    });

    for _ in 0..6 {
        state = run_iteration(state);
    }

    state.values().filter(|s| **s).count()
}

type Vector4 = (i32, i32, i32, i32);
type State4 = HashMap<Vector4, bool>;

fn get_neighbours_4(v: Vector4) -> Vec<Vector4> {
    let mut neighbours: Vec<Vector4> = Vec::with_capacity(81);

    for x in (v.0 - 1)..(v.0 + 2) {
        for y in (v.1 - 1)..(v.1 + 2) {
            for z in (v.2 - 1)..(v.2 + 2) {
                for w in (v.3 - 1)..(v.3 + 2) {
                    if x != v.0 || y != v.1 || z != v.2 || w != v.3 {
                        neighbours.push((x, y, z, w));
                    }
                }
            }
        }
    }

    neighbours
}

fn count_active_neighbours_4(state: &State4, v: Vector4) -> usize {
    get_neighbours_4(v).iter().filter(|nv| *state.get(nv).unwrap_or(&false)).count()
}

fn run_iteration_4(state: State4) -> State4 {
    let mut next_state: State4 = State4::new();

    let min_x = state.keys().map(|v| v.0).min().unwrap();
    let max_x = state.keys().map(|v| v.0).max().unwrap();
    let min_y = state.keys().map(|v| v.1).min().unwrap();
    let max_y = state.keys().map(|v| v.1).max().unwrap();
    let min_z = state.keys().map(|v| v.2).min().unwrap();
    let max_z = state.keys().map(|v| v.2).max().unwrap();
    let min_w = state.keys().map(|v| v.3).min().unwrap();
    let max_w = state.keys().map(|v| v.3).max().unwrap();

    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                for w in (min_w - 1)..(max_w + 2) {
                    let v = (x, y, z, w);
                    let active_neighbours = count_active_neighbours_4(&state, v);

                    let cur_state: bool = *state.get(&v).unwrap_or(&false);
                    if cur_state {
                        if active_neighbours == 2 || active_neighbours == 3 {
                            next_state.insert(v, true);
                        } else {
                            next_state.insert(v, false);
                        }
                    } else {
                        if active_neighbours == 3 {
                            next_state.insert(v, true);
                        } else {
                            next_state.insert(v, false);
                        }
                    }
                }
            }
        }
    }

    next_state
}

fn part2(data: &Vec<String>) -> usize {
    let mut state: State4 = State4::new();

    data.iter().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                state.insert((x as i32, y as i32, 0, 0), true);
            }
        });
    });

    for _ in 0..6 {
        state = run_iteration_4(state);
    }

    state.values().filter(|s| **s).count()
}

fn main() {
    let input: Vec<String> = vec![
        "##....#.",
        "#.#..#..",
        "...#....",
        "...#.#..",
        "###....#",
        "#.#....#",
        ".#....##",
        ".#.###.#"
    ].iter().map(|s| s.to_string()).collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let neighbours = get_neighbours((0, 0, 0));
        assert_eq!(neighbours.len(), 26);
        assert!(neighbours.iter().find(|(x, y, z)| *x == 0 && *y == 0 && *z == 0).is_none());
    }

    #[test]
    fn test_example() {
        let input: Vec<String> = vec![
            ".#.",
            "..#",
            "###",
        ].iter().map(|s| s.to_string()).collect();

        assert_eq!(part1(&input), 112);
        assert_eq!(part2(&input), 848);
    }
}