use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;
pub fn run(filename: &str) {
    let mut total: u64 = 0;
    {
        let _i = RaiiTime::new();
        let mut phase_two_started = false;
        let mut ranges = Vec::new();

        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            if phase_two_started {
                let input: i64 = line.parse().unwrap();
                match ranges.binary_search(&input) {
                    Ok(_) => total += 1,
                    Err(x) => {
                        // odd index indicates that it's higher than the lower bound
                        // but less than an upper bound
                        if x % 2 != 0 {
                            total += 1;
                        }
                    }
                }
            } else {
                if line.is_empty() {
                    phase_two_started = true;
                }
                let halves = line.split("-").collect::<Vec<_>>();
                if halves.len() != 2 {
                    continue;
                }
                let low: i64 = halves[0].parse().unwrap();
                let high: i64 = halves[1].parse().unwrap();

                if ranges.len() == 0 {
                    ranges.push(low);
                    ranges.push(high);
                    continue;
                }
                let first = ranges.binary_search(&low);
                let second = ranges.binary_search(&high);
                // Both endpoints are not found
                if first.is_err() && second.is_err() {
                    let low_pos = first.unwrap_err();
                    let high_pos = first.unwrap_err();
                    if low_pos == high_pos {
                        // Even index means its within a currently existing range
                        if low_pos % 2 == 0 {
                            ranges.insert(low_pos, low);
                            ranges.insert(low_pos + 1, high)
                        }
                        continue;
                    } else {
                    }
                }
            }
        }

        for b in ranges {
            println!("{:?}", b);
        }
    }
    println!("Total: {}", total);
}
