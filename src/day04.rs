use std::collections::HashMap;
use std::ops;

use itertools::Itertools;
use regex::Regex;

use crate::day::Day;

pub struct Day4 {
    passports: Vec<Passport>
}

impl Day4 {
    pub fn new(input: String) -> Self {
        let mut passports = Vec::new();
        for data in input.split("\n\n") {
            passports.push(Passport::parse(data))
        }
        return Day4 {
            passports
        };
    }
}

struct Passport {
    values: HashMap<String, String>
}

impl Passport {
    fn parse(data: &str) -> Self {
        let values: HashMap<_, _> = data.split_whitespace()
            .map(|kv| kv.split(':').next_tuple().unwrap())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .into_iter()
            .collect();
        return Passport {
            values
        };
    }

    pub fn has_keys(&self, required: &Vec<&str>) -> bool {
        for &key in required {
            if !self.values.contains_key(key) {
                return false;
            }
        }
        return true;
    }

    pub fn is_valid<P>(&self, key: &str, validator: P) -> bool
        where P: FnOnce(&String) -> bool {
        return self.values.get(key).map(validator).unwrap_or(false);
    }
}

impl ops::Index<&str> for Passport {
    type Output = String;

    fn index(&self, index: &str) -> &Self::Output {
        return self.values.get(index).unwrap();
    }
}

impl Day for Day4 {
    fn task1(&self) -> String {
        let required_keys = Vec::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

        let valid = self.passports.iter()
            .filter(|x| x.has_keys(&required_keys))
            .count();
        return format!("Valid passports: {}", valid);
    }

    fn task2(&self) -> String {
        let mut valid = 0;
        let height_regex = Regex::new(r"(\d+)(cm|in)").unwrap();
        for passport in &self.passports {
            if passport.is_valid(
                "byr",
                |v| v.parse::<usize>()
                    .map(|year| year >= 1900 && year <= 2002)
                    .unwrap_or(false))
                && passport.is_valid(
                "iyr",
                |v| v.parse::<usize>()
                    .map(|year| year >= 2010 && year <= 2020)
                    .unwrap_or(false))
                && passport.is_valid(
                "eyr",
                |v| v.parse::<usize>()
                    .map(|year| year >= 2020 && year <= 2030)
                    .unwrap_or(false))
                && passport.is_valid(
                "hgt",
                |v| {
                    return height_regex.captures(v)
                        .map(|c| {
                            let value = c[1].parse::<usize>().unwrap_or(0);
                            let unit = &c[2];
                            return if unit == "cm" {
                                value >= 150 && value <= 193
                            } else {
                                value >= 59 && value <= 76
                            };
                        })
                        .unwrap_or(false);
                })
                && passport.is_valid(
                "hcl",
                |v| v.chars().next().filter(|c| *c == '#').is_some()
                    && hex::decode(&v[1..v.len()]).is_ok())
                && passport.is_valid(
                "ecl",
                |v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**v))
                && passport.is_valid(
                "pid",
                |v| v.len() == 9 && v.parse::<usize>().is_ok())
            {
                valid += 1;
            }
        }

        return format!("Valid passports: {}", valid);
    }
}
