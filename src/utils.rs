use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    lines
}

pub fn read_groups<F: FnMut(&String) -> ()>(filename: &str, mut group_fn: F) {
    let lines = read_lines(filename);

    let mut builder: String = String::new();

    for line in lines {
        if line.is_empty() {
            group_fn(&builder);
            builder.clear();
        } else {
            builder.push(' ');
            builder.push_str(line.as_str());
        }
    }

    // Clear out the last one
    group_fn(&builder);
}

