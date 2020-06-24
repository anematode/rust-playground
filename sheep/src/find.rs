use colored::*;
use rand::Rng;
use std::io::{self, Write};

const COLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ROWS: &str = "1234567890";

#[derive(Debug)]
enum GuessResult {
    Found,
    Distance(f32),
    OutOfBoundsErr,
    OutOfGuessesErr,
}

#[derive(Debug)]
enum Cell {
    Tested,
    Untested,
}

/// So I can change it later if needed
type Unsigned = u8;

#[derive(Debug)]
struct Grid {
    width: Unsigned,
    height: Unsigned,
    location: (Unsigned, Unsigned),
    attempts: Vec<(Unsigned, Unsigned)>,
    max_attempts: usize,
}

impl Grid {
    fn new(width: Unsigned, height: Unsigned, max_attempts: usize) -> Grid {
        let mut rng = rand::thread_rng();
        Grid {
            width: width,
            height: height,
            location: (rng.gen_range(0, width), rng.gen_range(0, height)),
            attempts: Vec::with_capacity(max_attempts),
            max_attempts: max_attempts,
        }
    }

    fn guesses_left(&self) -> usize {
        self.max_attempts - self.attempts.len()
    }

    fn make_guess(&mut self, x: Unsigned, y: Unsigned) -> GuessResult {
        if self.guesses_left() <= 0 {
            GuessResult::OutOfGuessesErr
        } else if x >= self.width || y >= self.height {
            GuessResult::OutOfBoundsErr
        } else if (x, y) == self.location {
            GuessResult::Found
        } else {
            self.attempts.push((x, y));
            GuessResult::Distance(
                (x as f32 - self.location.0 as f32).hypot(y as f32 - self.location.1 as f32),
            )
        }
    }

    fn get_cell(&self, x: Unsigned, y: Unsigned) -> Cell {
        if self.attempts.contains(&(x, y)) {
            Cell::Tested
        } else {
            Cell::Untested
        }
    }

    fn print(&self, show_sheep: bool) {
        let mut string = String::new();
        string.push_str(format!(" {}\n", COLS[0..(self.width as usize)].yellow().bold()).as_str());
        for (y, char) in ROWS.chars().enumerate() {
            if y as Unsigned >= self.height {
                break;
            }
            string.push_str(&format!("{}", char.to_string().cyan().bold()).to_string());
            for x in 0..self.width {
                if show_sheep && (x, y as Unsigned) == self.location {
                    string.push_str(&"$".bold().to_string());
                } else {
                    let mut cell = match self.get_cell(x, y as Unsigned) {
                        Cell::Tested => ' ',
                        Cell::Untested => '?',
                    }
                    .to_string();
                    if show_sheep {
                        cell = cell.dimmed().to_string();
                    }
                    string.push_str(cell.as_str());
                }
            }
            string.push('\n');
        }
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
    let mut game = Grid::new(16, 9, 20);
    println!("\n{} Find it.\n", "The sheep hides.".bold());
    loop {
        game.print(false);
        println!(
            "Give a row and column, such as 1A ({} guesses left)",
            game.guesses_left()
        );
        let input = get_input();
        println!("");
        let mut chars = input.chars();
        if let (Some(row_char), Some(col_char)) = (chars.next(), chars.next()) {
            if let (Some(row), Some(col)) = (ROWS.find(row_char), COLS.find(col_char)) {
                match game.make_guess(col as Unsigned, row as Unsigned) {
                    GuessResult::Found => {
                        println!("{} You found the sheep!", "[:)]".green());
                        break;
                    }
                    GuessResult::Distance(dist) => {
                        println!(
                            "{} {}",
                            "[!]".yellow(),
                            match dist {
                                // Can't use float ranges anymore for some reason
                                n if n <= 1.0 => {
                                    "BAAAAAAAAA!!!!! (It's almost as if the sheep is right next to you!)"
                                }
                                n if n <= 2.0 => "BAAAAAAAA!!!!",
                                n if n <= 3.0 => "BAAAAAA!!!",
                                n if n <= 5.0 => "BAAAA!!",
                                n if n <= 10.0 => "BAAA!",
                                n if n <= 15.0 => "Baa!",
                                n if n <= 20.0 => "Baa.",
                                _ => "*bah*",
                            }
                        );
                    }
                    GuessResult::OutOfBoundsErr => {
                        println!("{} Out of bounds.", "[your error]".red());
                    }
                    GuessResult::OutOfGuessesErr => {
                        println!("{} No more guesses left.", "[your error]".red());
                        break;
                    }
                }
            } else {
                println!(
                    "{} Couldn't determine row/column from input.",
                    "[your error]".red()
                );
            }
        } else {
            println!("{} Give two characters.", "[your error]".red());
        }
        if game.guesses_left() <= 0 {
            println!(
                "{} You're out of guesses. The evasive sheep stands victorious!",
                "[:(]".red()
            );
            break;
        }
    }
    game.print(true);
}
