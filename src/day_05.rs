use std::cmp::Ordering;
use std::collections::HashSet;
use crate::util;

pub fn part_one() {
    let input = util::read(2024, 5, false);

    let rules: Vec<&str> = input.lines()
        .take_while(|line| !line.trim().is_empty())
        .collect();

    let updates: Vec<&str> = input.lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .collect();

    let mut total: u128 = 0;

    let mut index: HashSet<&str> = HashSet::new();
    rules.iter().for_each(|entry| { index.insert(entry); });

    updates.iter().for_each(|update| {
        let parts: Vec<&str> = update.split(",").collect();
        let mut gud = true;
        'outer: for i in 0..parts.len() {
            let to_verify = parts[i];
            for j in 0..parts.len() {
                let part = parts[j];
                if i == j {
                    continue;
                }

                let valid_key = if j > i {
                    format!("{}|{}", to_verify, part)
                } else {
                    format!("{}|{}", part, to_verify)
                };

                if index.contains(valid_key.as_str()) {
                    // ok
                    continue;
                }

                let invalid_key = if j > i {
                    format!("{}|{}", part, to_verify)
                } else {
                    format!("{}|{}", to_verify, part)
                };

                if index.contains(invalid_key.as_str()) {
                    gud = false;
                    break 'outer;
                }
            }
        }
        if gud {
            // println!("Gud: [{:?}]", update);
            total += parts[parts.len() / 2].parse::<u128>().unwrap();
        } else {
            // println!("Bad: [{:?}]", update);
        }
    });

    println!("Total: {}", total);
}

pub fn part_two() {
    let input = util::read(2024, 5, false);

    let rules: Vec<&str> = input.lines()
        .take_while(|line| !line.trim().is_empty())
        .collect();

    let updates: Vec<&str> = input.lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .collect();

    let mut total: u128 = 0;

    let mut index: HashSet<&str> = HashSet::new();
    rules.iter().for_each(|entry| { index.insert(entry); });

    updates.iter().for_each(|update| {
        let mut parts: Vec<&str> = update.split(",").collect();
        let mut gud = true;
        'outer: for i in 0..parts.len() {
            let to_verify = parts[i];
            for j in 0..parts.len() {
                let part = parts[j];
                if i == j {
                    continue;
                }

                let valid_key = if j > i {
                    format!("{}|{}", to_verify, part)
                } else {
                    format!("{}|{}", part, to_verify)
                };

                if index.contains(valid_key.as_str()) {
                    // ok
                    continue;
                }

                let invalid_key = if j > i {
                    format!("{}|{}", part, to_verify)
                } else {
                    format!("{}|{}", to_verify, part)
                };

                if index.contains(invalid_key.as_str()) {
                    gud = false;
                    break 'outer;
                }
            }
        }
        if gud {
            // println!("Gud: [{:?}]", update);
            // total += parts[parts.len() / 2].parse::<u128>().unwrap();
        } else {
            // println!("Bad: [{:?}]", update);
            parts.sort_by(|a, b| {
                if index.contains(format!("{}|{}", a, b).as_str()) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            // println!("Bad: {:?}", parts);
            total += parts[parts.len() / 2].parse::<u128>().unwrap();
        }
    });

    println!("Total: {}", total);
}