use aoc2020::utils;
use crate::MatchResult::MatchWithFlip;

#[derive(Clone)]
struct Tile {
    id: usize,
    tile_data: Vec<Vec<char>>,
}

impl Tile {
    fn get_edges(&self) -> Vec<Vec<char>> {
        let top: Vec<char> = self.tile_data.first().unwrap().clone();
        let right: Vec<char> = self.tile_data.iter().map(|row| row.last().unwrap().clone()).collect();
        let bottom: Vec<char> = self.tile_data.last().unwrap().iter().rev().map(|c| c.clone()).collect();
        let left: Vec<char> = self.tile_data.iter().rev().map(|row| row.first().unwrap().clone()).collect();

        vec![top, right, bottom, left]
    }

    fn print(&self) {
        self.tile_data.iter().for_each(|r| {
            r.iter().for_each(|c| print!("{}", c));
            println!();
        })
    }
}

fn parse(data: &Vec<String>) -> Vec<Tile> {
    let mut lines = data.iter();
    let mut tiles: Vec<Tile> = vec![];

    loop {
        let first_line = lines.next();
        if first_line.is_none() {
            break;
        }

        let id_string: String = first_line.unwrap()[5..9].to_string();
        let id = id_string.parse::<usize>().unwrap();

        let mut tile_data: Vec<Vec<char>> = Vec::with_capacity(10);
        for _ in 0..10 {
            tile_data.push(lines.next().unwrap().chars().collect());
        }

        tiles.push(Tile { id, tile_data });

        // Skip newline
        lines.next();
    }

    tiles
}

#[derive(PartialEq)]
enum MatchResult {
    Match,
    MatchWithFlip,
}

fn find_matching_edge<'a>(tiles: &'a Vec<Tile>, source_edge: &Vec<char>, source_id: usize)
                          -> Option<(&'a Tile, usize, MatchResult)>
{
    let other_tiles = tiles.iter()
        .filter(|t| t.id != source_id);

    for t in other_tiles {
        // Does this tile match this edge, with or without a flip?
        for (i, e) in t.get_edges().iter().enumerate() {
            if e == source_edge {
                return Some((t, i, MatchResult::Match));
            }

            let mut reversed_e = e.clone();
            reversed_e.reverse();

            if reversed_e == *source_edge {
                return Some((t, i, MatchResult::MatchWithFlip));
            }
        }
    }

    None
}

fn has_matching_edge(tiles: &Vec<Tile>, source_edge: &Vec<char>, source_id: usize) -> bool {
    find_matching_edge(tiles, source_edge, source_id).is_some()
}

fn find_edges_with_matches(tile: &Tile, tiles: &Vec<Tile>) -> Vec<usize> {
    tile.get_edges().iter()
        .enumerate()
        .filter(|(_, e)| {
            has_matching_edge(tiles, *e, tile.id)
        })
        .map(|(i, _)| i)
        .collect()
}

fn find_corners(tiles: &Vec<Tile>) -> Vec<&Tile> {
    tiles.iter().map(|t| {
        let edge_indexes_with_matches = find_edges_with_matches(t, tiles);
        if edge_indexes_with_matches.len() == 2 {
            Some(t)
        } else {
            None
        }
    })
        .filter(Option::is_some)
        .map(|v| v.unwrap())
        .collect()
}

fn rotate_tile_90_degrees(tile: &Tile) -> Tile {
    let size = tile.tile_data.len();
    let mut new_tile = Tile {
        id: tile.id,
        tile_data: vec![],
    };

    for i in 0..size {
        new_tile.tile_data.push((0..size).rev().map(|j| {
            tile.tile_data[j][i]
        }).collect());
    }

    new_tile
}

fn flip(tile: &Tile) -> Tile {
    Tile {
        id: tile.id,
        tile_data: tile.tile_data.iter().rev().map(|row| row.clone()).collect(),
    }
}

fn part1(tiles: &Vec<Tile>) -> usize {
    find_corners(tiles).iter().map(|t| t.id).product()
}

fn get_aligned_top_left_corner(tiles: &Vec<Tile>) -> Tile {
    let first_corner = find_corners(tiles)[0];
    let first_corner_inside_edges = find_edges_with_matches(first_corner, tiles);

    println!("Inside edges {:?}", first_corner_inside_edges);

    let mut num_rotations = 2;
    if first_corner_inside_edges != vec![0, 3] {
        num_rotations = (5 - first_corner_inside_edges[0]) % 4
    }
    let mut first_corner_aligned = first_corner.clone();
    (0..num_rotations).for_each(|_| first_corner_aligned = rotate_tile_90_degrees(&first_corner_aligned));

    first_corner_aligned
}

fn get_aligned_tile(tile: &Tile, edge_to_rotate_to_top: usize, flip_needed: bool) -> Tile {
    let mut next_tile = tile.clone();

    // 3 -> 1
    // 2 -> 2
    // 1 -> 3
    // 0 -> 0
    let number_rotations = (4 - edge_to_rotate_to_top) % 4;
    println!("Doing {} rotations to align tile", number_rotations);

    for _ in 0..number_rotations {
        next_tile = rotate_tile_90_degrees(&next_tile);
    }

    if !flip_needed {
        println!("Flipping tile");

        next_tile = flip(&next_tile);
        next_tile = rotate_tile_90_degrees(&next_tile);
        next_tile = rotate_tile_90_degrees(&next_tile);
    }

    next_tile
}

fn build_row(first_tile: Tile, remaining_tiles: &mut Vec<Tile>) -> Vec<Tile> {
    println!("> Starting row with {}", first_tile.id);

    remaining_tiles.retain(|t| t.id != first_tile.id);
    let mut result: Vec<Tile> = vec![first_tile];

    loop {
        let parent_tile = &result.last().unwrap();
        // Grab the right edge
        let parent_edge = &parent_tile.get_edges()[1];
        let aligned_next_tile: Tile;

        {
            println!("Parent tile: ");
            parent_tile.print();
            println!();

            println!("Edge we're looking for {:?} from {}", parent_edge, parent_tile.id);
            let next_match = find_matching_edge(remaining_tiles, parent_edge, parent_tile.id);

            if next_match.is_none() {
                // Hit the right side
                println!("< Done going right");
                break;
            }

            let (matched_tile, edge_index, match_type) = next_match.unwrap();

            println!(" Found match to the right {} [{}, {}]", matched_tile.id, edge_index, match_type == MatchWithFlip);

            println!("Matched tile: ");
            matched_tile.print();
            println!();

            let edge_to_rotate_to_top = (edge_index + 1) % 4; // 1 -> 2, 2 -> 3, 3 -> 0, 0 -> 1
            aligned_next_tile = get_aligned_tile(matched_tile, edge_to_rotate_to_top, match_type == MatchResult::MatchWithFlip);

            println!("Aligned next tile: ");
            matched_tile.print();
            println!();
        }

        remaining_tiles.retain(|t| t.id != aligned_next_tile.id);
        result.push(aligned_next_tile);
    }

    result
}

fn part2(tiles: &Vec<Tile>) -> usize {
    let first_corner = get_aligned_top_left_corner(tiles);
    let mut remaining_tiles = tiles.clone();

    let mut grid: Vec<Vec<Tile>> = vec![build_row(first_corner, &mut remaining_tiles)];

    // Find the first tile in each row and then build the row from there
    loop {
        let parent_tile = &grid.last().unwrap()[0];
        // Grab the bottom edge
        let parent_edge = &parent_tile.get_edges()[2];

        println!("Parent tile: ");
        parent_tile.print();
        println!();
        println!("Edge we're looking for {:?} from {}", parent_edge, parent_tile.id);

        let next_match = find_matching_edge(tiles, parent_edge, parent_tile.id);

        if next_match.is_none() {
            println!("Done going down");
            // Hit the bottom left corner
            break;
        }

        let (matched_tile, edge_index, match_type) = next_match.unwrap();
        println!("Found next line going down {} ({} {})", matched_tile.id, edge_index, match_type == MatchWithFlip);

        println!("Matched tile: ");
        matched_tile.print();
        println!();

        let aligned_next_tile = get_aligned_tile(matched_tile, edge_index, match_type == MatchResult::MatchWithFlip);

        println!("Aligned next tile: ");
        matched_tile.print();
        println!();

        grid.push(build_row(aligned_next_tile, &mut remaining_tiles));
    }

    println!("Built a {} x {} grid", grid.len(), grid[0].len());
    grid.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{} ", c.id));
        println!();
    });

    0
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/20.txt");
    let tiles = parse(&data);

    println!("Part 1: {}", part1(&tiles));
    part2(&tiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let tile = Tile { id: 1, tile_data: vec![vec!['1', '2'], vec!['3', '4']] };
        let rotated_tile = rotate_tile_90_degrees(&tile);
        assert_eq!(rotated_tile.tile_data[0], vec!['3', '1']);
        assert_eq!(rotated_tile.tile_data[1], vec!['4', '2']);
    }

    #[test]
    fn test_flip() {
        let tile = Tile { id: 1, tile_data: vec![vec!['1', '2'], vec!['3', '4']] };
        let flipped_tile = flip(&tile);
        assert_eq!(flipped_tile.tile_data[0], vec!['3', '4']);
        assert_eq!(flipped_tile.tile_data[1], vec!['1', '2']);
    }

    #[test]
    fn test_get_aligned() {
        let tile = Tile { id: 1, tile_data: vec![vec!['1', '2'], vec!['3', '4']] };
        let rotate_tile_once = get_aligned_tile(&tile, 3, false);
        assert_eq!(rotate_tile_once.tile_data[0], vec!['3', '1']);
        assert_eq!(rotate_tile_once.tile_data[1], vec!['4', '2']);

        let rotate_tile_once_with_flip = get_aligned_tile(&tile, 3, true);
        assert_eq!(rotate_tile_once_with_flip.tile_data[0], vec!['1', '3']);
        assert_eq!(rotate_tile_once_with_flip.tile_data[1], vec!['2', '4']);
    }

    #[test]
    fn test_example() {
        let data: Vec<String> = utils::read_lines("./input_data/20.example.txt");
        let tiles = parse(&data);
        assert_eq!(tiles.len(), 9);
        assert_eq!(tiles[0].id, 2311);
        assert_eq!(tiles[0].tile_data.len(), 10);
        assert_eq!(tiles[0].tile_data[0].len(), 10);

        assert_eq!(part1(&tiles), 20899048083289);
    }

    #[test]
    fn test_example_part2() {
        let data: Vec<String> = utils::read_lines("./input_data/20.example.txt");
        let tiles = parse(&data);
        assert_eq!(tiles.len(), 9);
        assert_eq!(tiles[0].id, 2311);
        assert_eq!(tiles[0].tile_data.len(), 10);
        assert_eq!(tiles[0].tile_data[0].len(), 10);

        assert_eq!(part2(&tiles), 273);
    }
}