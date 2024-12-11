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

        let (success, fail_idx) = validate_test(&num_vec);

        if success {
            num_valid += 1;
        } else {
            if fail_idx > 0 {
                let mut copy_vec = num_vec.clone();

                copy_vec.remove(fail_idx - 1);

                let (l_success, _) = validate_test(&copy_vec);

                if l_success {
                    num_valid += 1;
                    continue;
                }
            }

            let mut copy_vec = num_vec.clone();

            copy_vec.remove(fail_idx);

            let (m_success, _) = validate_test(&copy_vec);

            if m_success {
                num_valid += 1;
                continue;
            }

            if fail_idx < num_vec.len() - 1 {
                let mut copy_vec = num_vec.clone();

                copy_vec.remove(fail_idx + 1);

                let (r_success, _) = validate_test(&copy_vec);

                if r_success {
                    num_valid += 1;
                    continue;
                }
            }
        }
    }

    println!("The number of valid tests is {num_valid}");
}

fn validate_test(nums: &[i16]) -> (bool, usize) {
    let mut dp: i16 = if nums.len() > 1 { nums[1] - nums[0] } else { 0 };
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];

        if (diff.abs() < 1 || diff.abs() > 3) || (diff.signum() != dp.signum()) {
            return (false, i);
        }

        dp = diff;
    }

    (true, 0)
}
