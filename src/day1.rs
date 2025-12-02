use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;

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
    let mut advanced2: i64 = 0;
    for mut line in lines.map_while(Result::ok) {
        let previous = lock_pos;
        if line.len() > 0 {
            match line.remove(0) {
                'L' => lock_pos -= line.parse::<i64>().unwrap(),
                'R' => lock_pos += line.parse::<i64>().unwrap(),
                _ => panic!("Invalid"),
            }
        }
        let quotient2 = lock_pos / 100;
        let quotient = lock_pos.div_euclid(100);
        let remainder = lock_pos.rem_euclid(100);

        if previous != 0 && lock_pos < 0 {
            advanced += 1;
        }
        advanced += quotient2.abs();
        advanced2 += quotient.abs();
        if remainder == 0 {
            total += 1;
            //advanced += 1;
            if quotient2.abs() < 1 {
                advanced += 1;
            }
        }
        if previous != 0 && lock_pos == 0 {
            advanced2 += 1;
        }
        if previous == 0 && quotient < 0 {
            advanced2 -= 1;
        }
        if quotient < 0 && remainder == 0 {
            advanced2 += 1;
        }

        println!("{}==={}", lock_pos, previous);
        println!("{}:{}", quotient2, quotient);
        println!("{};{}", advanced, advanced2);
        //assert!(advanced == advanced2);
        lock_pos = remainder;
        if ((advanced - advanced2).abs() > 2) {
            panic!("ahhh");
        }
    }
    let elapsted = now.elapsed().unwrap().as_micros();
    println!("Combination {}", total);
    println!("Strong Combination {}", advanced);
    println!("Strong Combination(new) {}", advanced2);
    println!("Time taken (micros) {}", elapsted);
}
