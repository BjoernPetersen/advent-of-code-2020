use std::fs;

use crate::day01::Day1;
use crate::day02::Day2;
use crate::day::Day;

mod day01;
mod day02;
mod input;

pub mod day;

fn read_input(day: i8) -> String {
    let path = format!("input/{:02}.txt", day);
    return fs::read_to_string(path.as_str()).expect(format!("File not found: {}", path).as_str());
}

pub fn get_day(num: i8) -> Box<dyn Day> {
    let input = read_input(num);
    return match num {
        1 => Box::from(Day1::new(input)),
        2 => Box::from(Day2::new(input)),
        _ => panic!("Unknown day: {}", num)
    };
}
