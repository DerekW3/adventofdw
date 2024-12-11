use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/input_day3.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut line_vec: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    line_vec.pop();
}

fn extract_mul(str_vec: &String) -> Vec<String> {
    let sub_vec: Vec<&str> = str_vec.split("mul(").collect();

    for chunk in sub_vec {}

    vec![]
}

fn validate_chunk(chunk: &str) -> bool {
    let mut partials: Vec<&str> = chunk.split(",").collect();

    if partials.len() != 2 {
        return false;
    }

    partials[1] = partials[1].split(")").next().unwrap();

    if partials[0].parse::<i64>().is_err() || partials[1].parse::<i64>().is_err() {
        return false;
    }

    true
}
