use rand::Rng;
use std::{
    fs,
    io::{self, Write},
};
use colored::*;

enum GuessResult {
    Found,
    Distance(f32),
    OutOfBoundsErr,
}

enum Cell {
    Tested,
    Untested,
}

/// So I can change it later if needed
type Unsigned = u8;

struct Grid {
    width: Unsigned,
    height: Unsigned,
    location: (Unsigned, Unsigned),
    attempts: Vec<(Unsigned, Unsigned)>,
}

impl Grid {
    fn new(width: Unsigned, height: Unsigned, max_attempts: usize) -> Grid {
        let mut rng = rand::thread_rng();
        Grid {
            width: width,
            height: height,
            location: (rng.gen_range(0, width), rng.gen_range(0, height)),
            attempts: Vec::with_capacity(max_attempts),
        }
    }

    fn make_guess(&mut self, x: Unsigned, y: Unsigned) -> GuessResult {
        if x >= self.width || y >= self.height {
            GuessResult::OutOfBoundsErr
        } else if (x, y) == self.location {
            GuessResult::Found
        } else {
            self.attempts.push((x, y));
            GuessResult::Distance((x as f32 - self.location.0 as f32).hypot(y as f32 - self.location.1 as f32))
        }
    }

    fn get_cell(&self, x: Unsigned, y: Unsigned) -> Cell {
        if self.attempts.contains(&(x, y)) {
            Cell::Tested
        } else {
            Cell::Untested
        }
    }

    fn print(&self) {
        let mut string = String::new();

        println!("{}", string);
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout()
        .flush()
        .expect(format!("{} Failed to flush stdout", "[repl error]".red()).as_str());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(format!("{} Failed to read line", "[repl error]".red()).as_str());

    input
}

pub fn main() {
    let game = Grid::new(16, 9, 10);
    loop {
        let input = get_input();
        //
    };
}
