use regex::Regex;
use crate::util;

pub fn part_one() {
    let input = util::read(2024, 3, false);

    let mul_pattern = r"mul\((\d+),(\d+)\)";
    let regex = Regex::new(mul_pattern).unwrap();
    let mut total: u128 = 0;

    input.lines().for_each(|line| {
        for item in regex.captures_iter(line) {
            let first = item[1].parse::<u32>().expect("Could not parse number");
            let second = item[2].parse::<u32>().expect("Could not parse number");
            let multiplied = first * second;
            total += multiplied as u128;
        }
    });

    println!("Total: {}", total);
}

pub fn part_two() {
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = util::read(2024, 3, false);

    let pattern = r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)";
    let regex = Regex::new(pattern).unwrap();
    let mut total: u128 = 0;

    let mut toggle = true;

    input.lines().for_each(|line| {
        for item in regex.captures_iter(line) {
            let full_match = item.get(0).unwrap().as_str();
            println!("Item: {}", full_match);
            if full_match == "do()" {
                toggle = true;
            } else if full_match == "don't()" {
                toggle = false;
            } else {
                if toggle {
                    let first = item[1].parse::<u32>().expect("Could not parse number");
                    let second = item[2].parse::<u32>().expect("Could not parse number");
                    let multiplied = first * second;
                    total += multiplied as u128;
                }
            }
        }
    });

    println!("Total: {}", total);
}