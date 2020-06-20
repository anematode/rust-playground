use std::fs;

mod parser {
    use std::str;
    use nom::{
        branch::alt,
        bytes::complete::{tag, is_a, is_not, take_while},
        character::complete::{char, multispace0, multispace1 as s},
        combinator::{map, opt},
        error::VerboseError,
        multi::separated_list,
        number::complete::le_i32,
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
        Err,
    };

    type Out<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

    // TODO: Better handle the error output in this function
    pub fn parse_expr<'a>(input: &'a str) -> Result<Vec<Command>, Err<VerboseError<&'a str>>> {
        match main(input) {
            Ok((_, output)) => Ok(output),
            Err(err) => Err(err),
        }
    }

    // Idk how to discard values so will just use false
    fn possessive_pronoun<'a>(input: &'a str) -> Out<'a, bool> {
        map(alt((
            tag("her"),
            tag("their"),
            tag("his"),
        )), |_| false)(input)
    }

    fn reflexive_pronoun<'a>(input: &'a str) -> Out<'a, bool> {
        map(alt((
            tag("herself"),
            tag("themself"),
            tag("himself"),
        )), |_| false)(input)
    }

    pub enum Type {
        Int,
    }

    fn parse_type<'a>(input: &'a str) -> Out<'a, Type> {
        alt((
            map(tag("Intellectual"), |_| Type::Int),
            map(tag("Intellectual"), |_| Type::Int), // HACK
        ))(input)
    }

    fn parse_varname<'a>(input: &'a str) -> Out<'a, &str> {
        map(tuple((
            is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            take_while(|c: char| c.is_lowercase()),
        )), |(char, rest): (&str, &str)| {
            let var_name = String::from(rest);
            var_name.insert_str(0, char);
            var_name.as_str()
        })(input)
    }

    pub enum Value {
        Int(i32),
    }

    pub enum Command<'a> {
        MakeVar(&'a str, Type),
        SetInt(&'a str, i32),
        Add(&'a str, &'a str, &'a str),
        Log(&'a str),
        Chain(&'a Vec<Command<'a>>),
        DoNothing,
    }

    // type Eee<'a> = fn(input: &'a str) -> Out<'a, &str>;
    // type Eee2<'a> = fn(input: &'a str) -> Out<'a, (&str, &str)>;
    // type Eee3<'a> = fn(input: &'a str) -> Out<'a, (&str, &str, &str)>;
    // type Bertha<'a> = fn(input: &'a str) -> Out<'a, Value>;
    // type Erg<'a> = fn(input: &'a str) -> Out<'a, i32>;

    // fn fufu<'a>(input: &'a str) -> Out<'a, Value> {
    //     map(terminated(|i: &'a str| le_i32(i), tuple((s, tag("units")))), |n: i32| Value::Int(n))(input)
    // }

    // error[E0277]: expected a `std::ops::Fn<(&str,)>` closure, found `impl std::ops::Fn<(&[u8],)>`
    //     --> src\nommy_nom_nommy_nom.rs:98:13
    fn parse_int_value<'a>(input: &'a str) -> Out<'a, Value> {
        // let one: Eee<'a> = tag("1");
        // let space: Eee<'a> = s;
        // let unit: Eee<'a> = tag("unit");
        // let units: Eee<'a> = tag("units");
        // let e3: Eee3<'a> = tuple((one, space, unit));
        // let e2: Eee2<'a> = tuple((space, units));
        // let b: Bertha<'a> = map(e3, |_: (&str, &str, &str)| Value::Int(1));
        // let c: Erg<'a> = terminated(le_i32, e2);
        // let d: Bertha<'a> = map(c, |n: i32| Value::Int(n));
        // let g: Bertha<'a> = alt((b, d));
        // g(input)

        // I think this is describing what `alt` is returning, but idk why the carets point at the
        // tuple. It seems the tuple is being seen as a &[u8] (byte array), but calling--- no. It's
        // expecting a tuple, so `alt` is expecting a &str tuple, but there is instead a &[u8]
        // tuple. Or maybe calling ...(input) is expecting a &str tuple. Also, why does the tuple
        // only have one item?

        // Apparently wrong because size not known at compile time or something
        // let eggest: (dyn Fn(&str,) -> Out<'a, Value>, dyn Fn(&str,) -> Out<'a, Value>) = (

        // Apparently wrong bc can't use impl for var type; I guess I need to use the actual implementor
        // let eggest: (impl Fn<(&str,)>, impl Fn<(&[u8],)>) = (

        // (impl std::ops::Fn<(&str,)>, for<'a> fn(&'a str) -> std::result::Result<(&'a str,
        // nommy_nom_nommy_nom::parser::Value), nom::internal::Err<nom::error::VerboseError<&'a
        // str>>> {nommy_nom_nommy_nom::parser::fufu})
        // ... -> Result<(&'a str, Value), Err>
        // ... for<'a> fn(&'a str) -> Out<'a, Value>
        // (Fn(&str,) -> Output<'a, Value>, Fn(&str,) -> Output<'a, Value>)
        // let eggest: (Fn(&str,) -> Out<'a, Value>, Fn(&str,) -> Out<'a, Value>) = (
        // //  ^
        //     map(tuple((tag("1"), s, tag("unit"))), |_: (&str, &str, &str)| Value::Int(1)),
        //     fufu,
        // );
        // eggest.uh();
        alt((
        //  ^
            map(tuple((tag("1"), s, tag("unit"))), |_: (&str, &str, &str)| Value::Int(1)),
            map(terminated(|i: &'a str| -> Out<'a, i32> {le_i32(i)}, tuple((s, tag("units")))), |n: i32| Value::Int(n)),
        ))(input)
    //  ^ expected an `Fn<(&str,)>` closure, found `impl std::ops::Fn<(&[u8],)>`

        // = help: the trait `std::ops::Fn<(&str,)>` is not implemented for `impl
        // std::ops::Fn<(&[u8],)>`
        // = note: required because of the requirements on the impl of `nom::branch::Alt<&str,
        // nommy_nom_nommy_nom::parser::Value, _>` for `(impl std::ops::Fn<(&str,)>, impl
        // std::ops::Fn<(&[u8],)>)`
    }

    // Tell Rust that it should be an Option<Value> at this point
    fn parse_init_value<'a>(input: &'a str) -> Out<'a, Option<Value>> {
        // `alt` does not support just one case, but just know that in the future, swap `opt` with
        // `alt` and wrap the existing case in a tuple.
        opt(preceded(tuple((
            char(','), s, tag("who"), s, tag("finds"), s, reflexive_pronoun,
            tag("worth"), s
        )), parse_int_value))(input)
        // |i: &str| -> Out<'a, Value> {
        //     alt((
        //         map(tuple((tag("1"), s, tag("unit"))), |_| Value::Int(1)),
        //         map(terminated(le_i32, tuple((s, tag("units")))), |n| Value::Int(n)),
        //     ))(i)
        // }
    }

    fn parse_sub_command<'a>(input: &'a str) -> Out<'a, Command> {
        alt((
            map(tuple((
                preceded(tuple((tag("consider"), s,)), parse_varname),
                preceded(tuple((char(','), s, tag("an"), s)), parse_type),
                parse_init_value,
            )), |(name, var_type, value)| match value {
                Some(value) => Command::Chain(&vec![
                    Command::MakeVar(name, var_type),
                    match value {
                        Value::Int(n) => Command::SetInt(name, n)
                    },
                ]),
                None => Command::MakeVar(name, var_type)
            }),
            map(tuple((
                preceded(tuple((tag("mash"), s)), parse_varname),
                preceded(tuple((s, tag("and"), s)), parse_varname),
                preceded(tuple((s, tag("violently,"), s, tag("producing"), s)), parse_varname),
            )), |(a, b, output)| Command::Add(a, b, output))
        ))(input)
    }

    fn parse_command<'a>(input: &'a str) -> Out<'a, Command> {
        terminated(alt((
            preceded(tuple((tag("Let"), s, tag("us"), tuple((s, tag("also"))), s)), parse_sub_command),
            map(delimited(
                tuple((tag("Allow"), s)),
                parse_varname,
                tuple((tag("declare"), s, possessive_pronoun, s, tag("worth"))),
            ), |varname| Command::Log(varname)),
        )), tuple((char('.'), s)))(input)
    }

    fn parse_comment<'a>(input: &'a str) -> Out<'a, Command> {
        map(delimited(
            char('('),
            alt((map(is_not(")"), |_| Command::DoNothing), parse_comment)),
            char(')')
        // Tells Rust that the output from delimited etc can be a &str rather than a command
        ), |_| Command::DoNothing)(input)
    }

    fn main<'a>(input: &'a str) -> Out<'a, Vec<Command>> {
        delimited(multispace0, separated_list(s, alt((parse_command, parse_comment))), multispace0)(input)
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
