use std::io::{self, Read};

/// https://codeforces.com/problemset/problem/894/A
///
/// given string of capital letters (aka ASCII, no UTF-8 bothersomeness), find
/// like yeah
///
/// apparently this passes, thansk bucket potato
/// https://codeforces.com/contest/894/submission/122537942
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
