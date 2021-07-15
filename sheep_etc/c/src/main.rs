use std::io::{stdin, Read};

/// https://codeforces.com/contest/1047/problem/A
///
/// 3 positive ints that sum to *n* where none of the ints are a multiple of 3
///
/// did I do this scuffedly??
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("can't read to string rip");
    let n = input.trim().parse::<usize>().expect("can't parse as int! bad!");
    if (n - 2) % 3 == 0 {
        // Subtract 1 from n - 2 so that it's no longer divisible by 3
        println!("1 2 {}", n - 3);
    } else {
        println!("1 1 {}", n - 2);
    }
}
