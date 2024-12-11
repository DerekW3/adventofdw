use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/input_day2.txt").expect("File not found.");
    let reader = BufReader::new(file);

    let string_vec: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut num_valid = 0;

    for line in &string_vec[0..string_vec.len() - 1] {
        let num_vec: Vec<i16> = line
            .split_whitespace()
            .map(|s| s.parse::<i16>().unwrap())
            .collect();

        num_valid += 1;
    }

    println!("The number of valid tests is {num_valid}");
}

fn validate_test(nums: Vec<i16>) -> (bool, usize) {
    let mut dp: i16 = 0;
    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];

        if (diff.abs() < 1 || diff.abs() > 3) || (i > 1 && (diff.signum() != dp.signum())) {
            return (false, i);
        }

        dp = diff;
    }

    (true, 0)
}
