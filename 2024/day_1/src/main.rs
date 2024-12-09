use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("data/location_ids.txt").expect("file not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut numbers_one: Vec<i64> = vec![];
    let mut numbers_two: Vec<i64> = vec![];

    println!("The last line is {}", lines[lines.len() - 1]);
    for line in &lines[0..lines.len() - 1] {
        let s_vec: Vec<&str> = line.split_whitespace().collect();
        let n_one: i64 = s_vec[0].parse().unwrap();
        let n_two: i64 = s_vec[1].parse().unwrap();
        println!("{n_one} {n_two}");
    }
}
