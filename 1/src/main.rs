use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input_numbers() -> Result<Vec<i32>, io::Error> {
    let lines = read_lines("./input.txt")?;

    Ok(lines
        .filter_map(Result::ok)
        .map(|line| line.parse::<i32>().unwrap())
        .collect())
}

fn main() {
    const TARGET_SUM: i32 = 2020;

    let input_numbers= read_input_numbers();

    if let Ok(mut numbers) = input_numbers {
        numbers.sort_unstable();

        let midpoint_index = numbers.iter().position(|n| n > &(TARGET_SUM / 2)).unwrap();
        let lower_half = &numbers[..midpoint_index];
        let upper_half = &numbers[midpoint_index..];

        for n in lower_half {
            let match_index = upper_half.binary_search(&(TARGET_SUM - n));
            if match_index.is_ok() {
                println!("Solution: {}", upper_half[match_index.unwrap()] * n);
                break;
            }
        }
    }
}
