use std::fs::read_to_string;
use std::path::Path;

use primitive_types::U256;

use crate::timing::RaiiTime;

pub fn run(filename: &str) {
    let mut total: u64 = 0;
    let mut first_iteration: u64 = 0;
    {
        let _i = RaiiTime::new();
        let mut big_boys = Vec::new();

        let wrong: u8 = '@' as u8;
        let mut max_len = 256;
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            let mut scratch: U256 = 0.into();
            let bytes = line.as_bytes();
            for fool in 0..256 {
                if fool != 0 {
                    scratch <<= 1;
                }
                if fool < bytes.len() {
                    let first = bytes[fool];
                    if first.eq(&wrong) {
                        scratch += 1.into();
                    }
                } else {
                    break;
                }
            }
            if bytes.len() < max_len {
                max_len = bytes.len();
            }
            big_boys.push(scratch);
        }
        max_len += 1;
        let mut current_removes = 1;
        let mut next_one = big_boys.clone();

        let one_constant: U256 = 1.into();
        let zero_constant: U256 = 0.into();

        while current_removes != 0 {
            current_removes = 0;
            for line_index in 0..big_boys.len() {
                let line_a;
                let line_c;
                let line_b = big_boys[line_index];
                if line_index > 0 {
                    line_a = big_boys[line_index - 1];
                } else {
                    line_a = 0.into();
                }
                if line_index + 1 < big_boys.len() {
                    line_c = big_boys[line_index + 1];
                } else {
                    line_c = 0.into();
                }

                let mut moving_mask: U256 = one_constant;
                for _ in 0..max_len {
                    let bit = line_b & moving_mask;

                    if bit != 0.into() {
                        let mut count = 0;
                        for i in [line_a, line_b, line_c] {
                            if (moving_mask & (i << 1)) > zero_constant {
                                count += 1;
                            }
                            if (moving_mask & (i >> 1)) > zero_constant {
                                count += 1;
                            }
                            if (moving_mask & i) > zero_constant {
                                count += 1;
                            }
                        }

                        if count <= 4 {
                            next_one[line_index] ^= moving_mask;
                            current_removes += 1;
                            total += 1;
                        }
                    }

                    moving_mask <<= 1;
                }
            }
            if first_iteration == 0 {
                first_iteration = total;
            }
            big_boys = next_one.to_owned();
        }
    }
    println!("Total Removes: {}", total);
    println!("First Removes: {}", first_iteration);
}
