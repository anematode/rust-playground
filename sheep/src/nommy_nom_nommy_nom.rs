use colored::*;
use std::{
    fs,
    io::{self, Write},
};

mod my_verbose {
    use nom::error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind};

    pub trait AsPtrable {
        fn as_ptr(&self) -> *const u8;
    }

    /// Idk how to add a method properly in Rust so will just copy code
    // #[cfg(feature = "alloc")]
    #[derive(Clone, Debug, PartialEq)]
    pub struct MyVerboseError<'a> {
        /// list of errors accumulated by `VerboseError`, containing the affected
        /// part of input data, and some context
        pub errors: Vec<(&'a str, VerboseErrorKind)>,
    }

    impl<'a> MyVerboseError<'a> {
        pub fn as_verbose_error(self) -> VerboseError<&'a str> {
            VerboseError {
                errors: self.errors,
            }
        }
    }

    // #[cfg(feature = "alloc")]
    impl<'a> ParseError<&'a str> for MyVerboseError<'a> {
        fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
            MyVerboseError {
                errors: vec![(input, VerboseErrorKind::Nom(kind))],
            }
        }

        fn append(input: &'a str, kind: ErrorKind, mut other: Self) -> Self {
            other.errors.push((input, VerboseErrorKind::Nom(kind)));
            other
        }

        fn from_char(input: &'a str, c: char) -> Self {
            MyVerboseError {
                errors: vec![(input, VerboseErrorKind::Char(c))],
            }
        }

        fn add_context(input: &'a str, ctx: &'static str, mut other: Self) -> Self {
            other.errors.push((input, VerboseErrorKind::Context(ctx)));
            other
        }

        /// https://github.com/Geal/nom/issues/887#issuecomment-476109207
        /// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b9c3c6037ff30de18a2f7942331168aa
        /// take the error from the branch that went the farthest
        fn or(self, other: Self) -> Self {
            let p1 = self.errors.first().unwrap().0.as_ptr();
            let p2 = other.errors.first().unwrap().0.as_ptr();
            if p1 <= p2 {
                other
            } else {
                self
            }
        }
    }
}

mod lang {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum Type {
        Int,
        Float,
    }

    impl Type {
        fn to_string(&self) -> String {
            match self {
                Type::Int => String::from("Int"),
                Type::Float => String::from("Float"),
            }
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    #[derive(Debug, Clone)]
    pub enum Value {
        Int(i32),
        Float(f32),
    }

    impl Value {
        fn to_string(&self) -> String {
            match self {
                Value::Int(value) => format!("Int({})", value),
                Value::Float(value) => format!("Float({})", value),
            }
        }
    }

    impl fmt::Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    #[derive(Debug, Clone)]
    pub enum ValuePair {
        Int(i32, i32),
        Float(f32, f32),
    }

    // Using String because functions cannot return &str (string slices) if a String was made in a
    // function. https://stackoverflow.com/a/43080280
    #[derive(Debug, Clone)]
    pub enum Command {
        MakeVar(String, Type),
        SetVar(String, Value),
        Add(String, String, String),
        Subtract(String, String, String),
        Multiply(String, String, String),
        Divide(String, String, String),
        Log(String),
        Chain(Vec<Command>),
        DoNothing,
    }

    impl Command {
        pub fn name(&self, i: usize) -> String {
            format!(
                "[#{}] {}",
                i,
                match self {
                    Command::MakeVar(..) => "MakeVar",
                    Command::SetVar(..) => "SetVar",
                    Command::Add(..) => "Add",
                    Command::Subtract(..) => "Subtract",
                    Command::Multiply(..) => "Multiply",
                    Command::Divide(..) => "Divide",
                    Command::Log(..) => "Log",
                    Command::Chain(..) => "Chain",
                    Command::DoNothing => "DoNothing",
                }
            )
        }
    }

    // impl Clone for Command {
    //     fn clone(&self) -> Self {
    //         match self {
    //             Command::MakeVar(name, var_type) => Command::MakeVar(name.clone(), var_type),
    //             Command::SetInt(name, value) => Command::SetInt(name.clone(), value),
    //             Command::Add(a, b, output) => Command::Add(a.clone(), b.clone(), output.clone()),
    //             Command::Log(name) => Command::Log(name.clone()),
    //             Command::Chain(commands) => Command::Chain(commands.to_vec()),
    //             Command::DoNothing => Command::DoNothing,
    //         }
    //     }
    // }

    impl ToString for Command {
        fn to_string(&self) -> String {
            match self {
                Command::MakeVar(name, var_type) => {
                    format!("MakeVar({} of type {})", name, var_type.to_string())
                }
                Command::SetVar(name, value) => format!("SetVar({} = {})", name, value.to_string()),
                Command::Add(a, b, output) => format!("Add({} + {} -> {})", a, b, output),
                Command::Subtract(a, b, output) => format!("Subtract({} - {} -> {})", a, b, output),
                Command::Multiply(a, b, output) => format!("Multiply({} * {} -> {})", a, b, output),
                Command::Divide(a, b, output) => format!("Divide({} / {} -> {})", a, b, output),
                Command::Log(name) => format!("Log({})", name),
                Command::Chain(commands) => {
                    let mut command_strs = String::new();
                    for command in commands {
                        command_strs.push_str(
                            format!("\n  {}", command.to_string().replace("\n  ", "\n    "))
                                .as_str(),
                        );
                    }
                    format!("Chain({}\n)", command_strs)
                }
                Command::DoNothing => String::new(),
            }
        }
    }

    pub fn commands_to_string(commands: &Vec<Command>) -> String {
        let mut command_strs = String::new();
        let mut first = true;
        for command in commands {
            if first {
                first = false;
            } else {
                command_strs.push('\n');
            }
            command_strs.push_str(command.to_string().as_str());
        }
        command_strs
    }
}

mod parser {
    use std::str;
    // use std::fmt;
    use super::lang::{Command, Type, Value};
    use super::my_verbose::MyVerboseError;
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag, take_while},
        character::complete::{char, digit1, multispace0, multispace1 as s},
        combinator::{map, map_res, opt},
        error::{context, convert_error},
        multi::{many0, many_till},
        sequence::{delimited, preceded, terminated, tuple},
        Err, // I think this is the same as nom::internal::Err
        IResult,
        Needed,
    };

    type Out<'a, T> = IResult<&'a str, T, MyVerboseError<'a>>;

    // TODO: Better handle the error output in this function
    pub fn parse<'a>(input: &'a str) -> Result<Vec<Command>, String> {
        let string = format!("{} <end>", input);
        let string_str = string.as_str();
        match main(string_str) {
            Ok((inp, output)) => {
                println!("{}", inp);
                Ok(output)
            }
            Err(err) => Err(match err {
                Err::Incomplete(needed) => match needed {
                    Needed::Unknown => "Need more data, but unsure how much.".to_string(),
                    Needed::Size(size) => format!("Need  precisely{} more data.", size),
                },
                Err::Error(verbose) => convert_error(string_str, verbose.as_verbose_error()),
                Err::Failure(verbose) => convert_error(string_str, verbose.as_verbose_error()),
            }),
        }
    }

    // Idk how to discard values so will just use false
    fn possessive_pronoun<'a>(input: &'a str) -> Out<'a, bool> {
        map(alt((tag("her"), tag("their"), tag("his"))), |_| false)(input)
    }

    fn reflexive_pronoun<'a>(input: &'a str) -> Out<'a, bool> {
        map(
            alt((tag("herself"), tag("themself"), tag("himself"))),
            |_| false,
        )(input)
    }

    fn parse_type<'a>(input: &'a str) -> Out<'a, Type> {
        context(
            "type",
            alt((
                map(tag("Intellectual"), |_| Type::Int),
                map(tag("Flotation Device"), |_| Type::Float),
            )),
        )(input)
    }

    fn parse_varname<'a>(input: &'a str) -> Out<'a, String> {
        context(
            "variable name",
            map(
                tuple((
                    is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                    take_while(|c: char| c.is_lowercase()),
                )),
                |(char, rest): (&'a str, &'a str)| -> String {
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
                },
            ),
        )(input)
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
    fn parse_init_value<'a>(input: &'a str) -> Out<'a, Value> {
        // `alt` does not support just one case, but just know that in the future, swap `opt` with
        // `alt` and wrap the existing case in a tuple.
        context(
            "initial value",
            preceded(
                tuple((
                    char(','),
                    s,
                    tag("who"),
                    s,
                    tag("finds"),
                    s,
                    reflexive_pronoun,
                    s,
                    tag("worth"),
                    s,
                )),
                alt((
                    //  ^
                    map(
                        tuple((tag("1"), s, tag("unit"))),
                        |_: (&str, &str, &str)| Value::Int(1),
                    ),
                    map(
                        terminated(
                            map_res(
                                tuple((opt(char('-')), digit1)),
                                |(minus, digits): (Option<char>, &'a str)| {
                                    let mut num = String::new();
                                    if let Some(_) = minus {
                                        num.push('-');
                                    }
                                    num.push_str(digits);
                                    num.parse::<i32>()
                                },
                            ),
                            tuple((s, tag("units"))),
                        ),
                        |n: i32| Value::Int(n),
                    ),
                    map(
                        tuple((tag("1"), s, tag("pizza"))),
                        |_: (&str, &str, &str)| Value::Float(1.0),
                    ),
                    map(
                        terminated(
                            map_res(
                                tuple((opt(char('-')), digit1, opt(tuple((char('.'), digit1))))),
                                |(minus, digits, decimal): (
                                    Option<char>,
                                    &'a str,
                                    Option<(char, &'a str)>,
                                )| {
                                    let mut num = String::new();
                                    if let Some(_) = minus {
                                        num.push('-');
                                    }
                                    num.push_str(digits);
                                    if let Some((_, decimals)) = decimal {
                                        num.push('.');
                                        num.push_str(decimals);
                                    }
                                    num.parse::<f32>()
                                },
                            ),
                            tuple((s, tag("pizzas"))),
                        ),
                        |n: f32| Value::Float(n),
                    ),
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
                )),
            ),
        )(input)
        // |i: &str| -> Out<'a, Value> {
        //     alt((
        //         map(tuple((tag("1"), s, tag("unit"))), |_| Value::Int(1)),
        //         map(terminated(le_i32, tuple((s, tag("units")))), |n| Value::Int(n)),
        //     ))(i)
        // }
    }

    fn parse_sub_command<'a>(input: &'a str) -> Out<'a, Command> {
        context(
            "\"Let us (also)...\" command",
            alt((
                map(
                    tuple((
                        preceded(tuple((tag("consider"), s)), parse_varname),
                        preceded(tuple((char(','), s, tag("an"), s)), parse_type),
                        map(parse_init_value, |s| Some(s)),
                    )),
                    |(name, var_type, value)| {
                        let name_str = String::from(name);
                        match value {
                            Some(value) => Command::Chain(vec![
                                Command::MakeVar(name_str.clone(), var_type),
                                Command::SetVar(name_str, value),
                            ]),
                            None => Command::MakeVar(name_str, var_type),
                        }
                    },
                ),
                map(
                    tuple((
                        preceded(tuple((tag("mash"), s)), parse_varname),
                        preceded(tuple((s, tag("and"), s)), parse_varname),
                        preceded(
                            tuple((s, tag("violently,"), s, tag("producing"), s)),
                            parse_varname,
                        ),
                    )),
                    |(a, b, output)| Command::Add(a, b, output),
                ),
                map(
                    tuple((
                        preceded(tuple((tag("remove"), s)), parse_varname),
                        preceded(tuple((s, tag("from"), s)), parse_varname),
                        preceded(tuple((char(','), s, tag("yielding"), s)), parse_varname),
                    )),
                    |(b, a, output)| Command::Subtract(a, b, output),
                ),
                map(
                    tuple((
                        preceded(tuple((tag("have"), s)), parse_varname),
                        preceded(tuple((s, tag("and"), s)), parse_varname),
                        preceded(
                            tuple((s, tag("reproduce,"), s, tag("birthing"), s)),
                            parse_varname,
                        ),
                    )),
                    |(a, b, output)| Command::Multiply(a, b, output),
                ),
            )),
        )(input)
    }

    fn parse_command<'a>(input: &'a str) -> Out<'a, Command> {
        context(
            "command",
            terminated(
                alt((
                    context(
                        "Let us (also) ...",
                        preceded(
                            tuple((tag("Let"), s, tag("us"), opt(tuple((s, tag("also")))), s)),
                            parse_sub_command,
                        ),
                    ),
                    context(
                        "Let ... chop up ...",
                        map(
                            tuple((
                                preceded(tuple((tag("Let"), s)), parse_varname),
                                preceded(tuple((s, tag("chop"), s, tag("up"), s)), parse_varname),
                                preceded(
                                    tuple((char(','), s, tag("resulting"), s, tag("in"), s)),
                                    parse_varname,
                                ),
                            )),
                            |(b, a, output)| Command::Divide(a, b, output),
                        ),
                    ),
                    context(
                        "Allow ... to declare ... worth",
                        map(
                            delimited(
                                tuple((tag("Allow"), s)),
                                parse_varname,
                                tuple((
                                    s,
                                    tag("to"),
                                    s,
                                    tag("declare"),
                                    s,
                                    possessive_pronoun,
                                    s,
                                    tag("worth"),
                                )),
                            ),
                            |varname| Command::Log(String::from(varname)),
                        ),
                    ),
                    context("Hm", map(tag("Hm"), |_| Command::DoNothing)),
                )),
                char('.'),
            ),
        )(input)
    }

    fn parse_comment<'a>(input: &'a str) -> Out<'a, Command> {
        let create_filler = || map(take_while(|c| c != '(' && c != ')'), |_| Command::DoNothing);
        context(
            "comment",
            map(
                delimited(
                    char('('),
                    tuple((
                        create_filler(),
                        many0(tuple((parse_comment, create_filler()))),
                    )),
                    // map(is_not(")"), |_| Command::DoNothing),
                    char(')'), // Tells Rust that the output from delimited etc can be a &str rather than a command
                ),
                |_| Command::DoNothing,
            ),
        )(input)
    }

    fn main<'a>(input: &'a str) -> Out<'a, Vec<Command>> {
        context(
            "main",
            preceded(
                multispace0,
                map(
                    many_till(
                        terminated(alt((parse_command, parse_comment)), s),
                        context("end", tag("<end>")),
                    ),
                    |(vec, _)| vec,
                ),
            ),
        )(input)
    }

    // pub fn debug<'a>(input: &'a str) {
    //     println!("[debug] {:?}", dbg_dmp(main, "uh")(input))
    // }
}

mod interpreter {
    use super::lang::{Command, Type, Value, ValuePair};
    use std::collections::HashMap;

    pub struct Error {
        message: String,
        trace: Vec<String>,
    }

    impl Error {
        pub fn new(command: String, msg: String) -> Error {
            Error {
                message: command,
                trace: vec![msg],
            }
        }
        pub fn add_trace(&mut self, command: String) {
            self.trace.push(command);
        }
    }

    impl ToString for Error {
        fn to_string(&self) -> String {
            let mut err_output = self.message.clone();
            for command in &self.trace {
                err_output.push_str("\n  ");
                err_output.push_str(command);
            }
            err_output
        }
    }

    fn verify_operation(
        vars: &HashMap<String, Value>,
        in_a: String,
        in_b: String,
        out: String,
    ) -> Result<ValuePair, String> {
        if vars.contains_key(&out) {
            // return Err(Error::new(command.name(i), ));
            return Err(format!(
                "{} already exists and thus cannot be produced once more.",
                out
            ));
        }
        match (vars.get(&in_a), vars.get(&in_b)) {
            (Some(Value::Int(a)), Some(Value::Int(b))) => Ok(ValuePair::Int(*a, *b)),
            (Some(Value::Float(a)), Some(Value::Float(b))) => Ok(ValuePair::Float(*a, *b)),
            (Some(a), Some(b)) => Err(format!(
                "{} is {}, while {} is {}, so a mash would not produce anything meaningful.",
                in_a, a, in_b, b,
            )),
            (None, None) => Err(format!("Who are {} and {}?", in_a, in_b)),
            (None, _) => Err(format!("Who is {}?", in_a)),
            (_, None) => Err(format!("Who is {}?", in_b)),
        }
    }

    pub struct State {
        // https://stackoverflow.com/a/26324805 String as key is better than &'a String ok I guess
        // that makes sense
        vars: HashMap<String, Value>,
    }

    impl State {
        pub fn new() -> State {
            State {
                vars: HashMap::new(),
            }
        }

        pub fn execute(&mut self, commands: Vec<Command>) -> Result<(), Error> {
            // Temporary variable needed to prevent borrowing something as a mutable more than
            // once etc.
            // Inspired by https://stackoverflow.com/a/37987197
            let mut vars_anchor = &mut self.vars;
            for (i, command) in commands.iter().enumerate() {
                // Give ownership to vars
                let mut vars = vars_anchor;
                match command {
                    Command::MakeVar(name, var_tpe) => {
                        if vars.contains_key(name) {
                            return Err(Error::new(
                                command.name(i),
                                format!("{} has already been introduced.", name),
                            ));
                        }
                        vars.insert(
                            name.to_string(),
                            match var_tpe {
                                Type::Int => Value::Int(0),
                                Type::Float => Value::Float(0.0),
                            },
                        );
                    }
                    Command::SetVar(name, value) => {
                        if !vars.contains_key(name) {
                            return Err(Error::new(command.name(i), format!("Who is {}?", name)));
                        }
                        vars.insert(name.to_string(), value.clone());
                    }
                    //  | Command::Subtract(in_a, in_b, output) | Command::Multiply(in_a, in_b, output) | Command::Divide(in_a, in_b, output)
                    Command::Add(in_a, in_b, output) => match verify_operation(
                        vars,
                        in_a.to_string(),
                        in_b.to_string(),
                        output.to_string(),
                    ) {
                        Ok(pair) => {
                            vars.insert(
                                output.to_string(),
                                match pair {
                                    ValuePair::Int(a, b) => Value::Int(a + b),
                                    ValuePair::Float(a, b) => Value::Float(a + b),
                                },
                            );
                        }
                        Err(err) => {
                            return Err(Error::new(command.name(i), err));
                        }
                    },
                    Command::Subtract(in_a, in_b, output) => match verify_operation(
                        vars,
                        in_a.to_string(),
                        in_b.to_string(),
                        output.to_string(),
                    ) {
                        Ok(pair) => {
                            vars.insert(
                                output.to_string(),
                                match pair {
                                    ValuePair::Int(a, b) => Value::Int(a - b),
                                    ValuePair::Float(a, b) => Value::Float(a - b),
                                },
                            );
                        }
                        Err(err) => {
                            return Err(Error::new(command.name(i), err));
                        }
                    },
                    Command::Multiply(in_a, in_b, output) => match verify_operation(
                        vars,
                        in_a.to_string(),
                        in_b.to_string(),
                        output.to_string(),
                    ) {
                        Ok(pair) => {
                            vars.insert(
                                output.to_string(),
                                match pair {
                                    ValuePair::Int(a, b) => Value::Int(a * b),
                                    ValuePair::Float(a, b) => Value::Float(a * b),
                                },
                            );
                        }
                        Err(err) => {
                            return Err(Error::new(command.name(i), err));
                        }
                    },
                    Command::Divide(in_a, in_b, output) => match verify_operation(
                        vars,
                        in_a.to_string(),
                        in_b.to_string(),
                        output.to_string(),
                    ) {
                        Ok(pair) => {
                            vars.insert(
                                output.to_string(),
                                match pair {
                                    ValuePair::Int(a, b) => Value::Int(a / b),
                                    ValuePair::Float(a, b) => Value::Float(a / b),
                                },
                            );
                        }
                        Err(err) => {
                            return Err(Error::new(command.name(i), err));
                        }
                    },
                    Command::Log(name) => {
                        if let Some(value) = vars.get(name) {
                            match value {
                                Value::Int(val) => println!("{}", val),
                                Value::Float(val) => println!("{}", val),
                            }
                        } else {
                            return Err(Error::new(command.name(i), format!("Who is {}?", name)));
                        }
                    }
                    Command::Chain(commands) => {
                        if let Err(mut err) = self.execute(commands.to_vec()) {
                            err.add_trace(command.name(i));
                            return Err(err);
                        }
                        vars = &mut self.vars;
                    }
                    Command::DoNothing => {}
                };
                // Toss ownership back to vars_anchor
                vars_anchor = vars;
            }
            Ok(())
        }
    }
}

// Relative to sheep/ (current directory)
const DEFAULT_PATH: &'static str = "./stuff/yesyes.txt";

pub fn yes(maybe_path: Option<&String>) {
    let default_path = String::from(DEFAULT_PATH);
    let path = maybe_path.unwrap_or(&default_path);
    let input = fs::read_to_string(path).unwrap();
    match parser::parse(&input) {
        Ok(parsed) => {
            println!(
                "{} (length {})\n{}",
                "[:)]".green().bold(),
                parsed.len(),
                lang::commands_to_string(&parsed)
            );
            let mut state = interpreter::State::new();
            if let Err(err) = state.execute(parsed) {
                println!("{} {}", "[:(]".red().bold(), err.to_string());
            }
        }
        Err(err) => println!("{} {}", "[:(]".red().bold(), err),
    };
    // parser::debug(&input);
}

pub fn repl() {
    let mut state = interpreter::State::new();
    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect(format!("{} Failed to flush stdout", "[repl error]".red()).as_str());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(format!("{} Failed to read line", "[repl error]".red()).as_str());
        match parser::parse(&input) {
            Ok(parsed) => {
                if let Err(err) = state.execute(parsed) {
                    println!("{} {}", "[runtime error]".red().bold(), err.to_string())
                }
            }
            Err(err) => println!("{} {}", "[syntax error]".red().bold(), err.to_string()),
        };
    }
}
