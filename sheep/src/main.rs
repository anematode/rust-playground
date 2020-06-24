mod eagle;
mod find;
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
            "repl" => nommy_nom_nommy_nom::repl(),
            "eagle" => eagle::strtest::main(),
            "gp5" => eagle::gp5_perf_test::main(),
            "silly" => eagle::silly::silly(),
            "find" => find::main(),
            _ => {
                println!("uhhh am confuse but nonetheless... {:?}", args);
            }
        };
    } else {
        println!(
            r#"
            Follow `cargo run` with a name:
            - cargo run loud
            - cargo run regex
            - cargo run nom
            - cargo run nt
            - cargo run repl
            - cargo run eagle
            - cargo run gp5
            - cargo run silly
            - cargo run find
        "#
        );
    }
}
