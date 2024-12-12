use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/input_day3.txt").expect("File not found");
    let reader = BufReader::new(file);

    let line_vec: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut result = 0;

    for line in line_vec {
        result += extract_mul(&line);
    }

    println!("The result is found to be {result}");
}

fn extract_mul(line: &str) -> i64 {
    let mut do_iter = line.match_indices("do()").map(|cmd| cmd.0);
    let mut dont_iter = line.match_indices("don't()").map(|cmd| cmd.0);

    let sub_vec: Vec<&str> = line.split("mul(").collect();

    let mut process = true;

    let mut res = 0;

    let mut do_cmd = do_iter.next().unwrap_or(line.len());
    let mut dont_cmd = dont_iter.next().unwrap_or(line.len());
    let mut idx = 0;

    for chunk in sub_vec {
        idx += chunk.len();

        while do_cmd < idx || dont_cmd < idx {
            if do_cmd <= dont_cmd {
                process = true;
                do_cmd = do_iter.next().unwrap_or(line.len());
            } else {
                process = false;
                dont_cmd = dont_iter.next().unwrap_or(line.len());
            }
        }

        if process && validate_chunk(chunk) {
            let exp = chunk.split(")").next().unwrap();

            let nums: Vec<i64> = exp
                .split(",")
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            res += nums[0] * nums[1];
        }
    }

    res
}

fn validate_chunk(chunk: &str) -> bool {
    let mut partials: Vec<&str> = chunk.split(",").collect();

    if partials.len() < 2 {
        return false;
    }

    partials[1] = partials[1].split(")").next().unwrap();

    if partials[0].parse::<i64>().is_err() || partials[1].parse::<i64>().is_err() {
        return false;
    }

    true
}
