use crate::day::Day;
use crate::input::parse_ints;

// foreach-loops go brrrrrrrrrrrrrrrrrrrrr
pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Self {
        Day1 { input }
    }
}

impl Day for Day1 {
    fn task1(&self) -> String {
        let numbers: Vec<_> = parse_ints(&self.input).collect();
        for n in &numbers {
            for m in &numbers {
                if n + m == 2020 {
                    return format!("{}", n * m);
                }
            }
        }
        panic!("Could not find solution")
    }

    fn task2(&self) -> String {
        let numbers: Vec<_> = parse_ints(&self.input).collect();
        for n in &numbers {
            for m in &numbers {
                for o in &numbers {
                    if n + m + o == 2020 {
                        return format!("{}", n * m * o);
                    }
                }
            }
        }
        panic!("Could not find solution");
    }
}
