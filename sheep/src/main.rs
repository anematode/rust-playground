mod hi;

use std::env;

fn loud() {
    println!("Hello, world!");
    hi::access_me();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(mode) = args.get(1) {
        match mode.as_str() {
            "loud" => loud(),
            _ => {
                println!("uhhh am confuse but nonetheless... {:?}", args);
            }
        }
    } else {
        println!("Give flag. Maybe like 'loud' or something.")
    }
}
