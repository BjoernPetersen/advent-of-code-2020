use crate::input::{parse_ints, read_input};

// foreach-loops go brrrrrrrrrrrrrrrrrrrrr

pub fn task1() {
    let numbers: Vec<_> = parse_ints(&read_input(1)).collect();
    for n in &numbers {
        for m in &numbers {
            if n + m == 2020 {
                println!("Result1: {}", n * m);
                return;
            }
        }
    }
}

pub fn task2() {
    let numbers: Vec<_> = parse_ints(&read_input(1)).collect();
    for n in &numbers {
        for m in &numbers {
            for o in &numbers {
                if n + m + o == 2020 {
                    println!("Result2: {}", n * m * o);
                    return;
                }
            }
        }
    }
}
