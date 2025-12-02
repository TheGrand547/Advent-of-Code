use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn run(filename: &str) {
    let file = match File::open(Path::new(filename)) {
        Err(x) => panic!("Failed to load file {}", x),
        Ok(x) => x,
    };

    let lines = std::io::BufReader::new(file).lines();
    let mut lock_pos: i64 = 50;
    let mut total: i64 = 0;
    let mut advanced: i64 = 0;
    for mut line in lines.map_while(Result::ok) {
        let previous = lock_pos;
        if line.len() > 0 {
            match line.remove(0) {
                'L' => lock_pos -= line.parse::<i64>().unwrap(),
                'R' => lock_pos += line.parse::<i64>().unwrap(),
                _ => panic!("Invalid"),
            }
        }
        let quotient = lock_pos / 100;
        let remainder = lock_pos.rem_euclid(100);
        if previous != 0 && lock_pos < 0 {
            advanced += 1;
        }
        advanced += quotient.abs();
        if remainder == 0 {
            total += 1;
            if quotient.abs() < 1 {
                advanced += 1;
            }
        }
        lock_pos = remainder;
    }
    println!("Combination {}", total);
    println!("Strong Combination {}", advanced);
}
