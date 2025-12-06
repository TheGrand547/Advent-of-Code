use std::fs::read_to_string;
use std::path::Path;

use crate::timing::RaiiTime;
pub fn run(filename: &str) {
    let mut total: u64 = 0;
    {
        let _i = RaiiTime::new();
        let mut phase_two_started = false;
        let mut ranges = Vec::new();

        for line in read_to_string(Path::new(filename))
            .unwrap_or_default()
            .lines()
        {
            if phase_two_started {
                let input: i64 = line.parse().unwrap();
                match ranges.binary_search_by(|x: &(i64, i64)| x.0.cmp(&input)) {
                    Ok(_) => total += 1,
                    Err(position) => {
                        if position + 1 < ranges.len() {
                            let current = ranges[position + 1];
                            if current.0 <= input && input <= current.1 {
                                total += 1;
                            }
                        }
                        if position < ranges.len() {
                            let current = ranges[position];
                            if current.0 <= input && input <= current.1 {
                                total += 1;
                            }
                        }
                        if position > 0 {
                            let current = ranges[position - 1];
                            if current.0 <= input && input <= current.1 {
                                total += 1;
                            }
                        }
                    }
                }
            } else {
                if line.is_empty() {
                    phase_two_started = true;
                }
                let halves = line.split("-").collect::<Vec<_>>();
                if halves.len() != 2 {
                    continue;
                }
                let mut low: i64 = halves[0].parse().unwrap();
                let mut high: i64 = halves[1].parse().unwrap();
                if ranges.len() == 0 {
                    ranges.push((low, high));
                    continue;
                }
                //println!("{:?}", ranges);
                match ranges.binary_search_by(|x| x.0.cmp(&low).then(high.cmp(&x.1))) {
                    // Could be inserted here, thus is greater than everything before it
                    Err(posiition) => {
                        if posiition == 0 {
                            ranges.insert(posiition, (low, high));
                            continue;
                        }
                        let mut removes = Vec::with_capacity(2);
                        let previous = ranges[posiition - 1];
                        //println!("Cap{}", line);
                        if previous.1 < low {
                            ranges.insert(posiition, (low, high));
                            continue;
                        }
                        // Rangse will be combined with the previous
                        else {
                            low = previous.0;
                            removes.push(posiition - 1);
                        }
                        //println!("Toe: {}", posiition);
                        if posiition > ranges.len() {
                            ranges.insert(posiition, (low, high));
                            continue;
                        }
                        if posiition == ranges.len() {
                            if removes.len() == 0 {
                                ranges.push((low, high));
                            }
                        } else {
                            //println!("Sleep");
                            let next = ranges[posiition];
                            if next == (low, high)
                            {
                                continue;
                            }
                            if next.0 > high {
                                ranges.insert(posiition, (low, high));
                                continue;
                            } else {
                                high = next.1;
                                removes.push(posiition);
                            }
                        }
                        //println!("!{}", line);
                        if removes.len() == 1 {
                            ranges.push((low, high));
                            ranges.swap_remove(removes[0]);
                            continue;
                        } else if removes.len() == 0 {
                            ranges.insert(posiition, (low, high));
                        } else {
                            //println!("Yikes");
                            // remove both, screm
                            ranges.push((low, high));
                            ranges.swap_remove(posiition);
                            ranges.remove(removes[0]);
                        }
                    }
                    _ => (),
                }
            }
        }

        for b in ranges {
            println!("{:?}", b);
        }
    }
    println!("Total: {}", total);
}
