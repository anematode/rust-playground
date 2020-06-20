use std::fs;

mod parser {
    use std::str;
    // use std::fmt;
    use nom::{
        branch::alt,
        bytes::complete::{tag, is_a, is_not, take_while},
        character::complete::{char, multispace0, multispace1 as s, digit1},
        combinator::{map, opt, map_res},
        error::{VerboseError, context, convert_error},
        multi::separated_nonempty_list,
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
        Err, // I think this is the same as nom::internal::Err
        Needed,
    };

    type Out<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

    // TODO: Better handle the error output in this function
    pub fn parse<'a>(input: &'a str) -> Result<Vec<Command>, String> {
        match main(input) {
            Ok((inp, output)) => {
                println!("{}", inp);
                Ok(output)
            },
            Err(err) => Err(match err {
                Err::Incomplete(needed) => match needed {
                    Needed::Unknown => "Need more data, but unsure how much.".to_string(),
                    Needed::Size(size) => format!("Need  precisely{} more data.", size),
                },
                Err::Error(verbose) => convert_error(input, verbose),
                Err::Failure(verbose) => convert_error(input, verbose),
            }),
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

    #[derive(Debug)]
    pub enum Type {
        Int,
    }

    impl ToString for Type {
        fn to_string(&self) -> String {
            match self {
                Type::Int => String::from("Int"),
            }
        }
    }

    fn parse_type<'a>(input: &'a str) -> Out<'a, Type> {
        context("type", alt((
            map(tag("Intellectual"), |_| Type::Int),
            map(tag("Intellectual"), |_| Type::Int), // HACK
        )))(input)
    }

    fn parse_varname<'a>(input: &'a str) -> Out<'a, String> {
        context("variable name", map(tuple((
            is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            take_while(|c: char| c.is_lowercase()),
        )), |(char, rest): (&'a str, &'a str)| -> String {
            // let owned_string: String = String::from(char);
            // return owned_string + rest
            let mut var_name = String::with_capacity(char.len() + rest.len());
            var_name.push_str(char);
            var_name.push_str(rest);
            var_name
            // let var_name_ref: &'a String = &var_name;
            // var_name_ref
            // // var_name.as_str()
            // &var_name[..]
            // format!("{}{}", char, rest)
        }))(input)
    }

    #[derive(Debug)]
    pub enum Value {
        Int(i32),
    }

    // impl ToString for Value {
    //     fn to_string(&self) -> String {
    //         match self {
    //             Value::Int(value) => format!("Int({})", value),
    //         }
    //     }
    // }

    // Using String because functions cannot return &str (string slices) if a String was made in a
    // function. https://stackoverflow.com/a/43080280
    #[derive(Debug)]
    pub enum Command {
        MakeVar(String, Type),
        SetInt(String, i32),
        Add(String, String, String),
        Log(String),
        Chain(Vec<Command>),
        DoNothing,
    }

    // impl ToString for Command {
    //     fn to_string(&self) -> String {
    //         match self {
    //             Command::MakeVar(name, var_type) => format!("MakeVar({} of type {})", name, var_type),
    //             Command::SetInt(name, value) => format!("SetInt({} = {})", name, value),
    //             Command::Add(a, b, output) => format!("Add({} + {} -> {})", a, b, output),
    //             Command::Log(name) => format!("Log({})", name),
    //             Command::Chain(commands) => format!("Chain(\n  {}\n)", (&commands[..]).join("\n  ")),
    //             Command::DoNothing => String::new(),
    //         }
    //     }
    // }

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
    // fn parse_int_value<'a>(input: &'a str) -> Out<'a, Value> {
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
        // (input)
    //  ^ expected an `Fn<(&str,)>` closure, found `impl std::ops::Fn<(&[u8],)>`

        // = help: the trait `std::ops::Fn<(&str,)>` is not implemented for `impl
        // std::ops::Fn<(&[u8],)>`
        // = note: required because of the requirements on the impl of `nom::branch::Alt<&str,
        // nommy_nom_nommy_nom::parser::Value, _>` for `(impl std::ops::Fn<(&str,)>, impl
        // std::ops::Fn<(&[u8],)>)`
    // }

    // Tell Rust that it should be an Option<Value> at this point
    fn parse_init_value<'a>(input: &'a str) -> Out<'a, Option<Value>> {
        // `alt` does not support just one case, but just know that in the future, swap `opt` with
        // `alt` and wrap the existing case in a tuple.
        context("initial value", opt(preceded(tuple((
            char(','), s, tag("who"), s, tag("finds"), s, reflexive_pronoun,
            tag("worth"), s
        )), alt((
        //  ^
            map(tuple((tag("1"), s, tag("unit"))), |_: (&str, &str, &str)| Value::Int(1)),
            map(terminated(map_res(digit1, |digits: &'a str| digits.parse::<i32>()), tuple((s, tag("units")))), |n: i32| Value::Int(n)),
            // map(terminated(|i: &'a str| -> Out<'a, i32> {
            //     // let urg: IResult<&[u8], i32, ParseError<&[u8]>> = le_i32(i.as_bytes());
            //     let urg = le_i32::<'a, VerboseError<&'a [u8]>>(i.as_bytes());
            //     // VerboseError is a (implements) ParseError
            //     match urg {
            //         Ok((i, o)) => match str::from_utf8(i) {
            //             Ok(s) => Ok((s, o)),
            //             Err(err) => Err(Err::Error(VerboseError{errors: vec![("", VerboseErrorKind::Context("Couldn't parse as "))]})),
            //         },
            //         // Err(err) => Err(Err::Error(VerboseError{errors: vec![("", VerboseErrorKind::Context("eee"))]})),
            //         Err(err) => Err(err),
            //     }
            // }, tuple((s, tag("units")))), |n: i32| Value::Int(n)),
        )))))(input)
        // |i: &str| -> Out<'a, Value> {
        //     alt((
        //         map(tuple((tag("1"), s, tag("unit"))), |_| Value::Int(1)),
        //         map(terminated(le_i32, tuple((s, tag("units")))), |n| Value::Int(n)),
        //     ))(i)
        // }
    }

    fn parse_sub_command<'a>(input: &'a str) -> Out<'a, Command> {
        context("\"Let us (also)...\" command", alt((
            map(tuple((
                preceded(tuple((tag("consider"), s,)), parse_varname),
                preceded(tuple((char(','), s, tag("an"), s)), parse_type),
                parse_init_value,
            )), |(name, var_type, value)| {
                let name_str = String::from(name);
                match value {
                    Some(value) => {
                        Command::Chain(vec![
                            Command::MakeVar(name_str.clone(), var_type),
                            match value {
                                Value::Int(n) => Command::SetInt(name_str, n)
                            },
                        ])
                    },
                    None => Command::MakeVar(name_str, var_type)
                }
            }),
            map(tuple((
                preceded(tuple((tag("mash"), s)), parse_varname),
                preceded(tuple((s, tag("and"), s)), parse_varname),
                preceded(tuple((s, tag("violently,"), s, tag("producing"), s)), parse_varname),
            )), |(a, b, output)| Command::Add(a, b, output))
        )))(input)
    }

    fn parse_command<'a>(input: &'a str) -> Out<'a, Command> {
        context("command", terminated(alt((
            context("Let us (also) ...", preceded(tuple((
                tag("Let"), s, tag("us"), opt(tuple((s, tag("also")))), s
            )), parse_sub_command)),
            context("Allow ... to declare ... worth", map(delimited(
                tuple((tag("Allow"), s)),
                parse_varname,
                tuple((tag("declare"), s, possessive_pronoun, s, tag("worth"))),
            ), |varname| Command::Log(String::from(varname)))),
        )), tuple((char('.'), s))))(input)
    }

    fn parse_comment<'a>(input: &'a str) -> Out<'a, Command> {
        context("comment", map(delimited(
            char('('),
            alt((parse_comment, map(is_not(")"), |_| Command::DoNothing))),
            char(')')
        // Tells Rust that the output from delimited etc can be a &str rather than a command
        ), |_| Command::DoNothing))(input)
    }

    fn main<'a>(input: &'a str) -> Out<'a, Vec<Command>> {
        context("main", delimited(multispace0, separated_nonempty_list(s, alt((
            parse_command, parse_comment
        ))), multispace0))(input)
    }

    // pub fn debug<'a>(input: &'a str) {
    //     println!("[debug] {:?}", dbg_dmp(main, "uh")(input))
    // }
}

// Relative to sheep/ (current directory)
const DEFAULT_PATH: &'static str = "./stuff/yesyes.txt";

pub fn yes(maybe_path: Option<&String>) {
    let default_path = String::from(DEFAULT_PATH);
    let path = maybe_path.unwrap_or(&default_path);
    let input = fs::read_to_string(path).unwrap();
    match parser::parse(&input) {
        Ok(parsed) => println!("[:)] {:?} (length {})", parsed, parsed.len()),
        Err(err) => println!("[:(] {}", err),
    };
    // parser::debug(&input);
}
