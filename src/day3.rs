use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;
use std::thread;
pub fn run(filename: &str) {
    let mut total: u64 = 0;
    let thread_count = std::thread::available_parallelism()
        .unwrap_or(std::num::NonZero::new(1).unwrap())
        .get();
    println!("Thread Count: {}", thread_count);
    {
        let _i = RaiiTime::new();

        let mut threads = Vec::new();
        let all_lines = read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        let chunked_up = all_lines.as_slice().chunks(2);
        //all_lines.chunk_by(thread_count);
        /*
        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
            .into_iter()*/
        {
            for chunk in chunked_up {
                let personal_chunk = chunk.to_vec();
                let temp = thread::spawn(move || {
                    let mut inner_total: u64 = 0;
                    for line in personal_chunk {
                        let mut search_vec: Vec<char> = line.to_string().chars().collect();
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
                        inner_total += out.parse::<u64>().unwrap()
                    }
                    inner_total
                });
                threads.push(temp);
            }
        }

        for thread in threads {
            total += thread.join().unwrap();
        }
        println!("Total: {}", total);
    }
}
