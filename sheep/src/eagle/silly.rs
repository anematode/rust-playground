// use rand::prelude::*;

pub fn random_int() -> i8 {
    loop {
        if rand::random::<i8>() == 0 {
            return 4;
        }
    }
    // Intellectual!
    // "any code following this expression is unreachable"
    // 6
    // "^ unreachable expression"
}

pub fn silly() {
    println!("{}", random_int())
}
