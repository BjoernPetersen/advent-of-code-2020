pub fn parse_ints<'a>(s: &'a str) -> impl Iterator<Item=i32> + 'a {
    return s.lines().map(|l: &str| { l.parse::<i32>().unwrap() });
}
