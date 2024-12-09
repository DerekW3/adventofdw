use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

        numbers_one.push(n_one);
        numbers_two.push(n_two);
        println!("{n_one} {n_two}");
    }

    let mut heap_one = BinaryHeap::new();
    let mut heap_two = BinaryHeap::new();

    let mut map_one = HashMap::new();
    let mut map_two = HashMap::new();

    for i in 0..numbers_one.len() {
        heap_one.push(Reverse(numbers_one[i]));
        heap_two.push(Reverse(numbers_two[i]));

        let count_one = map_one.entry(numbers_one[i]).or_insert(0);
        let count_two = map_two.entry(numbers_two[i]).or_insert(0);

        *count_one += 1;
        *count_two += 1;
    }

    let mut dist = 0;
    let mut similarity = 0;

    for (key, _) in map_one.into_iter() {
        let val = map_two.get(&key).cloned().unwrap_or(0);

        similarity += key * val;
    }

    while !heap_one.is_empty() && !heap_two.is_empty() {
        dist += (heap_one.pop().unwrap().0 - heap_two.pop().unwrap().0).abs();
    }

    println!("The distance is {dist} and the similarity is {similarity}");
}
