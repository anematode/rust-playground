use std::io::{self, Read};

/// https://codeforces.com/problemset/problem/894/A
fn main() {
    let bytes_result: Result<Vec<u8>, _> = io::stdin().bytes().collect();
    let bytes = bytes_result.expect("failed to read bytes or smth");
    println!(
        "{}",
        bytes
            .iter()
            .enumerate()
            .filter(|(_, maybe_a)| **maybe_a == b'A')
            .map(|(i, _)| bytes
                .iter()
                .take(i)
                .filter(|maybe_q| **maybe_q == b'Q')
                .count()
                * bytes
                    .iter()
                    .skip(i + 1)
                    .filter(|maybe_q| **maybe_q == b'Q')
                    .count())
            .sum::<usize>()
    );
}
