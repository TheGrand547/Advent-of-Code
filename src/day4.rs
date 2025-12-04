use std::fs::read_to_string;
use std::path::Path;

use primitive_types::U256;

use crate::timing::RaiiTime;

pub fn run(filename: &str) {
    let mut total: u64 = 0;
    {
        let _i = RaiiTime::new();
        let mut big_boys = Vec::new();

        let wrong: u8 = '@' as u8;
        let mut bad_len = 0;
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            //let first;

            let mut scratch: U256 = 0.into();
            let bytes = line.as_bytes();
            for fool in 0..256 {
                if fool < bytes.len() {
                    let first = bytes[fool];
                    if first.eq(&wrong) {
                        scratch += 1.into();
                    }
                } else {
                    //scratch <<= 256 - bytes.len();
                    scratch <<= 2;
                    break;
                }
                scratch <<= 1;
            }
            if bytes.len() > bad_len {
                bad_len = bytes.len();
            }
            //println!("{:#0130b}", scratch.low_u128());
            big_boys.push(scratch);
        }
    //    println!("{}", bad_len);
        bad_len = 0;

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

            let mut moving_mask: U256 = 1.into();
            moving_mask <<= bad_len;
            for _ in bad_len..256 {
                let bit = line_b & moving_mask;

                if bit != 0.into() {
                    let mut count = 0;
                    for i in [line_a, line_b, line_c] {
                        if (moving_mask & (i << 1)) > 0.into() {
                            count += 1;
                        }
                        if (moving_mask & (i >> 1)) > 0.into() {
                            count += 1;
                        }
                        if (moving_mask & i) > 0.into() {
                            count += 1;
                        }
                    }
                    // as line_b & line_b & moving_mask will always be 1, this will always be overcounted
                    //count -= 1;
                    //println!("{}", count);
                    if count <= 4 {
                        //bitmask_out |= moving_mask;
                        total += 1;
                    }
                }

                moving_mask <<= 1;
            }
        }
        //total += bitmask_out.count_ones() as u64;

        println!("Total: {}", total);
    }
}
