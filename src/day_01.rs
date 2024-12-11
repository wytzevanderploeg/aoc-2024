use std::cmp;
use crate::util;

pub fn part_one() {
    let input = util::read(2024, 1, false);
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        left.push(split[0].parse().expect("Failed to parse"));
        right.push(split[1].parse().expect("Failed to parse"));
    });

    left.sort();
    right.sort();

    let mut total: u128 = 0;
    for i in 0..left.len() {
        let left = left[i];
        let right = right[i];
        let distance = cmp::max(left, right) - cmp::min(left, right);
        total += distance as u128;
    }
    println!("Total: {}", total);
}

pub fn part_two() {
    let input = util::read(2024, 1, false);
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        left.push(split[0].parse().expect("Failed to parse"));
        right.push(split[1].parse().expect("Failed to parse"));
    });

    left.sort();
    right.sort();

    let mut total: u128 = 0;
    for i in 0..left.len() {
        let left = left[i];
        let mut count: u32 = 0;
        for j in 0..right.len() {
            let right = right[j];
            if right < left {
                continue;
            }
            if right > left {
                break;
            }
            count += 1;
        }
        total += (left * count) as u128;
    }
    println!("Total: {}", total);
}