mod hi;
mod nom_test;
mod nommy_nom_nommy_nom;
mod regex_test;
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
            "regex" => regex_test::weee(args.get(2)),
            "nom" => nommy_nom_nommy_nom::yes(args.get(2)),
            "nt" => nom_test::yes(args.get(2)),
            _ => {
                println!("uhhh am confuse but nonetheless... {:?}", args);
            }
        }
    } else {
        println!("Give flag. Maybe like 'loud' or something.")
    }
}
