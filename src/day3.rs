use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;
pub fn run(filename: &str) {
    let mut total: u64 = 0;
    {
        let _i = RaiiTime::new();
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            let mut search_vec: Vec<char> = line.chars().collect();
            let length = search_vec.len();
            let mut output = " ".repeat(length);
            let mut start = 0;
            for i in (0..12).rev() {
                let index;
                let mut largest: char = '0';
                let mut large_index = start;
                for zoom in start..length - i {
                    let current = search_vec.get(zoom).unwrap();
                    if *current > largest {
                        largest = *current;
                        large_index = zoom;
                    }
                }
                start = large_index;
                index = large_index;
                output.replace_range(index..(index + 1), largest.to_string().as_str());
                *search_vec.get_mut(index).unwrap() = '0';
            }
            let out: String = output.chars().filter(|x| !x.is_whitespace()).collect();
            total += out.parse::<u64>().unwrap()
        }
    }
    println!("Total: {}", total);
}
