use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DATE: Regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
}

const MONTHS: [&str; 12] = [
    "enero",
    "febrero",
    "marzo",
    "abril",
    "mayo",
    "juno",
    "julio",
    "agosto",
    "septiembre",
    "octubre",
    "noviembre",
    "deciembre",
];

const DEFAULT_INPUT: &'static str = "2012-03-14, 2013-01-01, 2014-07-05, 2020-13-01";

pub fn weee(maybe_dates: Option<&String>) {
    let default_input = String::from(DEFAULT_INPUT);
    let text = maybe_dates.unwrap_or(&default_input);
    for cap in DATE.captures_iter(text) {
        match cap[2].parse::<usize>() {
            Ok(month @ 1..=12) => println!("{} de {}, {}", &cap[3], MONTHS[month - 1], &cap[1]),
            _ => println!("Invalid date."),
        }
    }
}
