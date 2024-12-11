use std::cmp;
use crate::util;

pub fn part_one() {
    let input = util::read(2024, 2, false);

    let data: Vec<Vec<u32>> = parse_data(&input);

    let mut count = 0;
    for line in data {
        let mut safe = true;
        let increasing = line[1] > line[0];

        for i in 1..line.len() {
            let left = line[i - 1];
            let right = line[i];
            let distance = cmp::max(left, right) - cmp::min(left, right);

            if increasing && left > right {
                safe = false;
                break;
            }
            if !increasing && right > left {
                safe = false;
                break;
            }
            if distance < 1 || distance > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            count += 1;
        }
    }

    println!("Safe: {}", count);
}

pub fn part_two() {
    let input = util::read(2024, 2, false);

    let data: Vec<Vec<u32>> = parse_data(&input);

    let mut count = 0;
    for line in data {
        let mut is_safe = safe(&line);
        if is_safe {
            count += 1;
            continue;
        } else {
            for i in 0..line.len() {
                let omitted = &omit_idx(&line, i);
                is_safe = safe(&omitted);
                if is_safe {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe: {}", count);
}

fn parse_data(input: &String) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn safe(line: &Vec<u32>) -> bool {
    let increasing = line[1] > line[0];

    for i in 1..line.len() {
        let left = line[i - 1];
        let right = line[i];
        let distance = cmp::max(left, right) - cmp::min(left, right);

        if increasing && left > right {
            return false;
        }
        if !increasing && right > left {
            return false;
        }
        if distance < 1 || distance > 3 {
            return false;
        }
    }

    true
}

fn omit_idx(input: &[u32], idx: usize) -> Vec<u32> {
    input
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != idx)
        .map(|(_, &value)| value)
        .collect()
}