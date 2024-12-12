use crate::util;

pub fn part_one() {
    let input = util::read(2024, 7, false);

    let values: Vec<(u128, Vec<u128>)> = input.lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();
            let total: u128 = split[0].trim().parse::<u128>().unwrap();
            let numbers: Vec<u128> = split[1].trim()
                .split_whitespace()
                .map(|item| item.parse::<u128>().unwrap())
                .collect();

            return (total, numbers);
        })
        .collect();

    let mut total = 0;
    for value in values {
        let sum = value.0;
        let numbers = value.1;
        let mut matched = false;

        let mut stack = vec![
            (1, Operation::multiply(numbers[0], numbers[1])),
            (1, Operation::plus(numbers[0], numbers[1]))
        ];

        while !stack.is_empty() {
            let stmt = stack.pop().unwrap();
            let idx = stmt.0;
            let op = stmt.1;

            let result = op.exec();

            if ((idx + 1) == numbers.len()) && result == sum {
                matched = true;
                continue;
            }

            if result > sum {
                continue;
            }

            if (idx + 1) < numbers.len() {
                stack.push((idx + 1, Operation::multiply(result, numbers[idx + 1])));
                stack.push((idx + 1, Operation::plus(result, numbers[idx + 1])));
            }
        }

        if matched {
            total += sum;
        }
    }
    println!("Total: {}", total);
}

pub fn part_two() {
    let input = util::read(2024, 7, false);

    let values: Vec<(u128, Vec<u128>)> = input.lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();
            let total: u128 = split[0].trim().parse::<u128>().unwrap();
            let numbers: Vec<u128> = split[1].trim()
                .split_whitespace()
                .map(|item| item.parse::<u128>().unwrap())
                .collect();

            return (total, numbers);
        })
        .collect();

    let mut total = 0;
    for value in values {
        let sum = value.0;
        let numbers = value.1;
        let mut matched = false;

        let mut stack = vec![
            (1, Operation::multiply(numbers[0], numbers[1])),
            (1, Operation::plus(numbers[0], numbers[1])),
            (1, Operation::concat(numbers[0], numbers[1]))
        ];

        while !stack.is_empty() {
            let stmt = stack.pop().unwrap();
            let idx = stmt.0;
            let op = stmt.1;

            let result = op.exec();

            if ((idx + 1) == numbers.len()) && result == sum {
                matched = true;
                continue;
            }

            if result > sum {
                continue;
            }

            if (idx + 1) < numbers.len() {
                stack.push((idx + 1, Operation::multiply(result, numbers[idx + 1])));
                stack.push((idx + 1, Operation::plus(result, numbers[idx + 1])));
                stack.push((idx + 1, Operation::concat(result, numbers[idx + 1])));
            }
        }

        if matched {
            total += sum;
        }
    }
    println!("Total: {}", total);
}

#[derive(Debug)]
struct Operation<T> {
    name: String,
    operator: fn(left: T, right: T) -> T,
    left: T,
    right: T
}

impl Operation<u128> {
    fn multiply(left: u128, right: u128) -> Self {
        Operation {
            name: "*".to_string(),
            operator: |left, right| left * right,
            left,
            right
        }
    }

    fn plus(left: u128, right: u128) -> Self {
        Operation {
            name: "+".to_string(),
            operator: |left, right| left + right,
            left,
            right
        }
    }

    fn concat(left: u128, right: u128) -> Self {
        Operation {
            name: "||".to_string(),
            operator: |left, right| format!("{}{}", left, right).parse::<u128>().unwrap(),
            left,
            right
        }
    }

    fn exec(&self) -> u128 {
        (self.operator)(self.left, self.right)
    }
}