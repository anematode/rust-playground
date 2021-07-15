/*
echo "7 3
abacaba
1 3
2 5
1 7" | cargo run --bin song
*/

use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    stdin
        .read_line(&mut String::new())
        .expect("can't read first line");
    // The song, consisting of n lowercase letters of English letters
    let mut song = String::new();
    stdin.read_line(&mut song).expect("can't read second line");

    for line_result in stdin.lock().lines() {
        let line = line_result.expect("why is this a result anyways");
        let mut question = line
            .split(" ")
            .map(|part| part.parse::<usize>().expect("can't parse as int. sad!"));
        let lower = question.next().expect("missing lower bound");
        let upper = question.next().expect("missing upper bound");
        println!(
            "{}",
            song.chars()
                .skip(lower - 1)
                .take(upper - lower + 1)
                .map(|char| char as u8 - b'a' + 1)
                .sum::<u8>()
        );
    }
}
