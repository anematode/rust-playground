mod hi;

use std::env;

fn loud() {
    println!("Hello, world!");
    hi::access_me();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let maybe_mode = args.get(1);
    match maybe_mode {
        Some(mode) => match mode.as_str() {
            "loud" => loud(),
            _ => {
                println!("uhhh am confuse but nonetheless... {:?}", args);
            }
        },
        None => println!("Give flag. Maybe like 'loud' or something.")
    }
}
