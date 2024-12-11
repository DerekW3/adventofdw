use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/input_day3.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut line_vec: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    line_vec.pop();
}
