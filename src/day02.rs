use itertools::Itertools;

use crate::day::Day;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new(input: String) -> Self {
        Day2 { input }
    }
}

struct Rule {
    char: char,
    min: usize,
    max: usize,
}

impl Rule {
    pub fn from_str(rule_str: &str) -> Self {
        let (range, char_str) = rule_str.split(' ').next_tuple().unwrap();
        let (min_str, max_str) = range.split('-').next_tuple().unwrap();
        return Rule {
            char: char_str.chars().nth(0).unwrap(),
            min: min_str.parse::<usize>().unwrap(),
            max: max_str.parse::<usize>().unwrap(),
        };
    }
}

fn parse(line: &str) -> (Rule, &str) {
    let (rule_str, pass) = line.split(": ").next_tuple()
        .expect(&format!("Invalid input line {}", line));
    let rule = Rule::from_str(rule_str);
    return (rule, pass);
}

impl Rule {
    fn accepts_first(&self, password: &str) -> bool {
        let occurrence = password.chars().filter(|c| *c == self.char).count();
        return occurrence >= self.min && occurrence <= self.max;
    }

    fn accepts_second(&self, password: &str) -> bool {
        return password.char_indices()
            .take(self.max)
            .filter(|(i, _)| *i == self.max - 1 || *i == self.min - 1)
            .filter(|(_, c)| c == &self.char)
            .exactly_one()
            .is_ok();
    }
}

impl Day for Day2 {
    fn task1(&self) -> String {
        let valid_count = self.input.lines()
            .map(parse)
            .filter(|(r, p)| r.accepts_first(p))
            .count();
        return format!("Valid passwords: {}", valid_count);
    }

    fn task2(&self) -> String {
        let valid_count = self.input.lines()
            .map(parse)
            .filter(|(r, p)| r.accepts_second(p))
            .count();
        return format!("Valid passwords: {}", valid_count);
    }
}
