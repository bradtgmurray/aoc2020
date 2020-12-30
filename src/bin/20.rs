use aoc2020::utils;

struct Tile {
    id: usize,
    tile_data: Vec<Vec<char>>,
}

impl Tile {
    fn get_edges(&self) -> Vec<Vec<char>> {
        vec![
            self.tile_data[0].clone(),
            [0..9].iter.map(|i| tile_data[i].unwrap()[9].unwrap()).collect(),
            self.tile_data[9].clone(),
            [0..9].iter.map(|i| tile_data[i].unwrap()[0].unwrap()).collect(),
        ]
    }
}

fn parse(data: &Vec<String>) -> Vec<Tile> {
    let mut lines = data.iter();
    let mut tiles: Vec<Tile> = vec![];

    loop {
        let mut first_line = lines.next();
        if first_line.is_none() {
            break;
        }

        let mut first_line_chars = first_line.unwrap().chars();
        first_line_chars.take(5); // Skip "Tile "
        let id_string: String = first_line_chars.take(4).collect();
        let id = id_string.parse::<usize>().unwrap();

        tiles.push(Tile {
            id,
            tile_data: lines.take(10).map(|l| l.chars().collect()).collect(),
        });
    }

    tiles
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/20.txt");
    let tiles = parse(&data);
}