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
            let original: Vec<char> = line.chars().collect();
            let mut str_line = line.to_string().clone();
            let mut ordered = original.clone();
            ordered.sort();
            let length = ordered.len();
            if length >= 2 {
                let joltage: u64;
                /*
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
                */
                let mut scratch: String = line.to_string();
                let mut index = 0;
                let mut putter = " ".repeat(line.len());
                let mut blast = Vec::with_capacity(length);
                while blast.len() < str_line.len() {
                    blast.push(0);
                }
                let mut fuck = ordered
                    .clone()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                for i in (0..12).rev() {
                    let current = ordered.pop().unwrap();
                    let cringe = fuck.pop();
                    let first_i = fuck.find(cringe.unwrap());
                    let index;
                    if first_i.is_none() || length - first_i.unwrap() < i {
                        index = str_line.find(current).unwrap();
                    } else {
                        index = str_line.rfind(current).unwrap();
                    }

                    //let index = str_line.rfind(current).unwrap();
                    str_line.replace_range(index..(index + 1), " ");
                    putter.replace_range(index..(index + 1), current.to_string().as_str());
                    println!("<{}>", str_line);
                    blast[index] = current.to_digit(10).unwrap();
                    //    putter.get_mut(index).unwrap() = current;
                    /*
                    let mut out = &mut putter.chars().nth(index).unwrap();
                    out = current;*/
                }
                let out: String = putter.chars().filter(|x| !x.is_whitespace()).collect();
                let outer: String = blast
                    .into_iter()
                    .filter(|x| *x != 0)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                while scratch.len() > 12 && false {
                    let current = ordered.get(index).unwrap();
                    let first = scratch.chars().nth(0).unwrap();
                    let second = scratch.chars().nth(1).unwrap();
                    if first.to_digit(10).unwrap() < second.to_digit(10).unwrap() {
                        scratch.remove(0);
                        //scratch = scratch.replacen(first, "", 1);
                        // I don't think this should make any difference, moreover it makes no sense
                        //index += 1;
                    } else {
                        match scratch.find(*current) {
                            Some(x) => {
                                scratch.remove(x);
                            }
                            None => (),
                        }
                        //scratch = scratch.replacen(*current, "", 1);
                        index += 1;
                    }
                    //println!("{}:{}", index, scratch);
                }
                //println!("{}", scratch);
                println!("{}", out);
                joltage = out.parse().unwrap();
                total += joltage;
            }
        }
    }
    println!("Total: {}", total);
}
