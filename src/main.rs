#![allow(dead_code)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod grid;
mod util;
mod day_08;

fn main() {
    let christmas_past = false;
    if christmas_past {
        day_01::part_one();
        day_01::part_two();
        day_02::part_one();
        day_02::part_two();
        day_03::part_one();
        day_03::part_two();
        day_04::part_one();
        day_04::part_two();
        day_05::part_one();
        day_05::part_two();
        day_06::part_one();
        day_06::part_two();
        day_07::part_one();
        day_07::part_two();
        day_08::part_one();
        day_08::part_two();
    }

}
