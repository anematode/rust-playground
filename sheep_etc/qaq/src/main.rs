use std::io::{self, Read};

fn main() {
    let bytes_result: Result<Vec<u8>, _> = io::stdin().bytes().collect();
    let bytes = bytes_result.expect("failed to read bytes or smth");
    let mut count = 0;
    for (i, maybe_a) in bytes.iter().enumerate() {
        if *maybe_a != b'A' {
            continue;
        }
        let mut left = 0;
        let mut right = 0;
        for maybe_q in bytes.iter().take(i) {
            if *maybe_q == b'Q' {
                left += 1;
            }
        }
        for maybe_q in bytes.iter().skip(i + 1) {
            if *maybe_q == b'Q' {
                right += 1;
            }
        }
        count += left * right;
    }
    println!("{}", count);
}
