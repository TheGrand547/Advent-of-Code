use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;
use std::fs::read_to_string;

pub fn run(filename: &str) {
    let now = SystemTime::now();
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
        //if line.len() > 0 {

        match line.remove(0) {
            'L' => lock_pos -= line.parse::<i64>().unwrap(),
            _ => lock_pos += line.parse::<i64>().unwrap_or(0),
        }
        //}
        let quotient = lock_pos.div_euclid(100);
        let remainder = lock_pos.rem_euclid(100);

        advanced += quotient.abs();
        let temp3: i64 = (remainder == 0).into();
        let temp4: i64 = (previous != 0 && lock_pos == 0).into();
        /*
        if remainder == 0 {
            total += 1;
        }
        if previous != 0 && lock_pos == 0 {
            advanced += 1;
        }*/
        let temp: i64 = (quotient < 0 && previous == 0).into();
        let temp2: i64 = (quotient < 0 && remainder == 0).into();
        advanced += temp2 + temp4 - temp;
        total += temp3;
        /*
        if quotient < 0 {
            if previous == 0 {
                advanced -= 1;
            }
            if remainder == 0 {
                advanced += 1;
            }
        }*/
        lock_pos = remainder;
    }
    let elapsted = now.elapsed().unwrap().as_micros();
    println!("Combination {}", total);
    println!("Strong Combination {}", advanced);
    println!("Time taken (micros) {}", elapsted);
}
