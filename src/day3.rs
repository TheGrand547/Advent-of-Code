use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;

pub fn run(filename: &str) {
    let mut total: u64 = 0;
    let mut prev: u64 = 0;
    {
        let _ = RaiiTime::new();
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            let mut original: Vec<char> = line.chars().collect();
            let mut ordered = original.clone();
            ordered.sort();
            let length = ordered.len();
            if length >= 2 {
                let mut joltage = 0;
                for offset in 1..(length - 1) {
                    let ultimate = *ordered.get(length - offset).unwrap();
                    let penultimate = *ordered.get(length - offset - 1).unwrap();
                    // Naive but true
                    if ultimate == penultimate {
                        let comical: u64 = ultimate.to_digit(10).unwrap().into();
                        joltage = comical * 11;
                        break;
                    }

                    let big_pos = line.find(ultimate).unwrap();
                    let small_pos = line.find(penultimate).unwrap();
                    if big_pos < small_pos {
                        let comical: u64 = ultimate.to_digit(10).unwrap().into();
                        let dos: u64 = penultimate.to_digit(10).unwrap().into();
                        joltage = comical * 10 + dos;
                        break;
                    }
                    if big_pos < length - 1 {
                        let eagle = original.get_mut(big_pos + 1..).unwrap();
                        eagle.sort();
                        let double_penultimate = *eagle.get(eagle.len() - 1).unwrap();
                        let comical: u64 = ultimate.to_digit(10).unwrap().into();
                        let dos: u64 = double_penultimate.to_digit(10).unwrap().into();
                        joltage = comical * 10 + dos;
                        break;
                    } else {
                        let comical: u64 = ultimate.to_digit(10).unwrap().into();
                        let dos: u64 = penultimate.to_digit(10).unwrap().into();
                        joltage = dos * 10 + comical;
                        break;
                    }
                }
                //println!("{}:{}:{}", total, total + joltage, joltage);
                total += joltage;
            }
        }
    }
    println!("Total: {}", total);
}
