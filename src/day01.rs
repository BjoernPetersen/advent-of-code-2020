use crate::day::Day;
use crate::input::parse_ints;

pub struct Day1 {
    input: Vec<i32>,
}

impl Day1 {
    pub fn new(input: String) -> Self {
        Day1 { input: parse_ints(&input).collect() }
    }
}

// foreach-loops go brrrrrrrrrrrrrrrrrrrrr
impl Day for Day1 {
    fn task1(&self) -> String {
        let numbers = &self.input;
        for n in numbers {
            for m in numbers {
                if n + m == 2020 {
                    return format!("{}", n * m);
                }
            }
        }
        panic!("Could not find solution")
    }

    fn task2(&self) -> String {
        let numbers = &self.input;
        for n in numbers {
            for m in numbers {
                for o in numbers {
                    if n + m + o == 2020 {
                        return format!("{}", n * m * o);
                    }
                }
            }
        }
        panic!("Could not find solution");
    }
}
