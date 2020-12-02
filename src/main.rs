extern crate advent20;

fn main() {
    // TODO accept cli arg
    let day = advent20::get_day(2);
    println!("Result 1: {}", day.task1());
    println!("Result 2: {}", day.task2());
}
