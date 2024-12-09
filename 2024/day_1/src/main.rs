use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("../data/location_ids.txt") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}
