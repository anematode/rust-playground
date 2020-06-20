use nom::{
    bytes::complete::tag,
    character::complete::multispace1 as s,
    sequence::tuple,
    IResult,
};

fn parser<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
  tuple((tag("Hello"), s))(input)
}

const DEFAULT_INPUT: &'static str = "Hellp sheep";

pub fn yes(maybe_input: Option<&String>) {
    let default_input = String::from(DEFAULT_INPUT);
    let input = maybe_input.unwrap_or(&default_input);
    println!("{:?}", parser(&input));
}
