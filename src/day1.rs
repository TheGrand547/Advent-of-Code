use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;

pub fn run(filename: &str) {
    let mut lock_pos: i64 = 50;
    let mut total: i64 = 0;
    let mut advanced: i64 = 0;
    {
        let _raii_time = RaiiTime::new();
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            let previous = lock_pos;
            // I've tried doing this with unchecked unrwapps but it seems to be slower, for some reason
            match line.chars().nth(0).unwrap() {
                'L' => lock_pos -= line.get(1..).unwrap().parse::<i64>().unwrap(),
                _ => lock_pos += line.get(1..).unwrap().parse::<i64>().unwrap(),
            }
            let quotient = lock_pos.div_euclid(100);
            let remainder = lock_pos.rem_euclid(100);

            advanced += quotient.abs();
            let zero_remainder: i64 = (remainder == 0).into();
            let zero_hit: i64 = (previous != 0 && lock_pos == 0).into();

            let overcount: i64 = (quotient < 0 && previous == 0).into();
            let undercount: i64 = (quotient < 0 && remainder == 0).into();
            advanced += undercount + zero_hit - overcount;
            total += zero_remainder;

            lock_pos = remainder;
        }
    }
    println!("Combination {}", total);
    println!("Strong Combination {}", advanced);
}
