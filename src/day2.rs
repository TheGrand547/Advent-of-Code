use crate::timing::RaiiTime;
use std::path::Path;

use std::fs::read_to_string;

use std::collections::HashMap;
use std::thread;
pub fn run(filename: &str) {
    let mut invalid_total: usize = 0;
    let mut double_invalid: usize = 0;

    {
        let _raii_time = RaiiTime::new();
        let mut handles = vec![];
        for element in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
        {
            let temp = element.to_string();
            let current_handle = thread::spawn(move || {
                let mut local_total: usize = 0;
                let mut local_invalid: usize = 0;
                let division: Vec<&str> = temp.split('-').collect();
                if division.len() == 2 {
                    let first = division[0].parse::<usize>().unwrap();
                    let last = division[1].parse::<usize>().unwrap();
                    //let mut mapping = HashMap::<usize, Vec<usize>>::new();

                    for i in first..=last {
                        let parsed = i.to_string();
                        let length = parsed.len();
                        if length <= 1 {
                            continue;
                        }
                        // Only even numbers need to be chcecked for the basic versions

                        if length % 2 == 0 {
                            let raw_bytes = parsed.as_str();
                            let half_length = length / 2;
                            if raw_bytes[0..half_length].eq(&raw_bytes[half_length..length]) {
                                local_invalid += i;
                            }
                        }
                        let bound_check = length / 2;
                        //let raw_bytes = parsed.as_str();
                        /*
                        if !mapping.contains_key(&length) {
                            let mut screaming = Vec::new();
                            for j in 1..=bound_check {
                                if length % j == 0 {
                                    screaming.push(j);
                                }
                            }
                            mapping.insert(length, screaming);
                        }
                        */
                        if length < 10 || true {
                            let raw_bytes = parsed.as_str();
                            for j in (1..=bound_check).rev() {
                                if length % j != 0 {
                                    continue;
                                }
                                let sub_string = &raw_bytes[0..j];
                                let sub_len = sub_string.len();
                                let mut index = sub_len;
                                while raw_bytes[index..].find(sub_string).unwrap_or(length) == 0 {
                                    index += sub_len;
                                }

                                if index == length {
                                    local_total += i;
                                    break;
                                }
                            }
                        } else {
                            let mut intneral_handles = vec![];
                            for offset in [0, 1] {
                                let copied = parsed.clone();
                                let local_handle = thread::spawn(move || {
                                    let mut result = false;
                                    for j in ((1 + offset)..=bound_check).step_by(2) {
                                        if length % j != 0 {
                                            continue;
                                        }
                                        let sub_string = &copied[0..j];
                                        let sub_len = sub_string.len();
                                        let mut index = sub_len;
                                        while copied[index..].find(sub_string).unwrap_or(length)
                                            == 0
                                        {
                                            index += sub_len;
                                        }
                                        if index == length {
                                            result = true;
                                            break;
                                        }
                                    }
                                    result
                                });
                                intneral_handles.push(local_handle);
                            }

                            let mut addition = false;
                            for thread in intneral_handles {
                                addition |= thread.join().unwrap();
                            }
                            if addition {
                                local_total += i;
                            }
                        }
                    }
                }

                (local_total, local_invalid)
            });
            handles.push(current_handle);
        }
        for handle in handles {
            let result = handle.join().unwrap();
            double_invalid += result.0;
            invalid_total += result.1;
        }
    }
    println!("# invalid ID's {}", invalid_total);
    println!("# extra invalid ID's {}", double_invalid);
}
