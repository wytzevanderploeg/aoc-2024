use std::fs;

pub fn read(year: i32, day: i32, test: bool) -> String {
    let day = format!("{:0>2}", day);
    let suffix = if test { "sample" } else { "input" };
    let path = format!("resources/{year}/{}/{}.txt", day, suffix);

    fs::read_to_string(path)
        .expect("File should be readable")
}

pub fn read_from_path(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File should be readable")
}

pub fn reverse(input: &str) -> String {
    String::from_iter(input.chars().rev())
}