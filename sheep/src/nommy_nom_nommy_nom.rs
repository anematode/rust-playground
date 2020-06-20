use std::fs;

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, multispace0, multispace1 as s, one_of},
        combinator::{cut, map, map_res, opt},
        error::{context, VerboseError},
        multi::many0,
        sequence::{delimited, preceded, terminated, tuple},
        IResult, Parser,
    };

    type Out<OutType, 'a> = IResult<&'a str, OutType, VerboseError<&'a str>>

    pub fn parse_expr<'a>(input: &'a str) -> Result<Expr, VerboseError<&'a str>> {
        match main(input) {
            Ok((_, output)) => Ok(output),
            Err(err) => Err(err)
        }
    }

    // Idk how to discard values so will just use false
    fn possessive_pronoun<'a>(input: &'a str) -> Out<bool, 'a> {
        map(alt((
            tag("her"),
            tag("their"),
            tag("his"),
        )), |_| false)(input)
    }

    fn reflexive_pronoun<'a>(input: &'a str) -> Out<bool, 'a> {
        map(alt((
            tag("herself"),
            tag("themself"),
            tag("himself"),
        )), |_| false)(input)
    }

    pub enum Type {
        Int,
    }

    fn parse_type<'a>(input: &'a str) -> Out<Type, 'a> {
        alt((
            map(tag("Integer"), |_| Type::Int)
        ))(input)
    }

    fn parse_varname<'a>(input: &'a str) -> Out<&String, 'a> {
        tuple((
            is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            take_while(is_lowercase),
        ))(input)
    }

    pub enum Command {
        MakeVar(&String, Type),
        Add(&String, &String, &String),
        Log(&String),
        Chain(&Vec<Command>),
    }

    fn parse_sub_command<'a>(input: &'a str) -> Out<Command, 'a> {
        alt((
            map(tuple((
                preceded(tuple((tag("consider"), s,)), parse_varname),
                preceded(tuple((char(','), s, tag("an"), s)), parse_type),
                opt(preceded((
                    char(','), s, tag("who"), s, tag("finds"), s, reflexive_pronoun,
                    tag("worth"), s
                ), alt((
                    map(tuple((tag("1"), s, tag("unit"))), |_| 1),
                    map(tuple((le_i32, s, tag("units"))), |(n, ..)| n),
                )))),
            )), |(name, var_type, value)| match value {
                Some(value) => Command::Chain(vec![
                    Command::MakeVar(name, var_type),
                    Command::SetVar(name, value)
                ]),
                None => Command::MakeVar(name, type)
            }),
            map(tuple((
                preceded(tuple((tag("mash"), s)), parse_varname),
                preceded(tuple((s, tag("and"), s)), parse_varname),
                preceded(tuple((s, tag("violently,"), s, tag("producing"), s)), parse_varname),
            )), |(a, b, output)| Command::Add(a, b, output))
        ))(input)
    }

    fn parse_command<'a>(input: &'a str) -> Out<Command, 'a> {
        terminated(alt((
            preceded(tuple((tag("Let"), s, tag("us"), tuple((s, opt("also"))), s)), parse_sub_command),
            map(delimited(
                tuple((tag("Allow"), s)),
                parse_varname,
                tuple((tag("declare"), s, possessive_pronoun, s, tag("worth"))),
            ), |varname| Command::Log(varname)),
        )), tuple((char('.'), s)))(input)
    }

    fn parse_comment<'a>(input: &'a str) -> Out<bool, 'a> {
        map(delimited(char('('), alt(is_not(")"), parse_comment), char(')')), |_| false)
    }

    fn main<'a>(input: &'a str) -> Out<&Vec<Command>, 'a> {
        separated_list(alt(parse_command, parse_comment))(input)
    }
}

// Relative to sheep/ (current directory)
const DEFAULT_PATH: &'static str = "./stuff/yesyes.txt";

pub fn yes(maybe_path: Option<&String>) {
    let default_path = String::from(DEFAULT_PATH);
    let path = maybe_path.unwrap_or(&default_path);
    let input = fs::read_to_string(path).unwrap();
    println!("{}", input);
}
