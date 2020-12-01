use std::fs;

pub fn read_input(day: i8) -> String {
    let path = format!("input/{:02}.txt", day);
    return fs::read_to_string(path.as_str()).expect(format!("File not found: {}", path).as_str());
}

pub fn parse_ints<'a>(s: &'a str) -> impl Iterator<Item=i32> + 'a {
    return s.lines().map(|l: &str| { l.parse::<i32>().unwrap() });
}
