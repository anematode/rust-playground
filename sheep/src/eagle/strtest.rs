//! String tests

/// If you give this fundtion a String, it'll OWN it. Whoever gave it the String will no longer be
/// able to use the String.
fn let_me_own_the_string(give_me: String) {
    println!("{}", give_me);
}

/// However, the function can give back the String as a return value so the caller function can
/// continue to use it.
fn i_will_own_the_string_and_use_it_then_return_it(give_me: String) -> String {
    println!("{}", give_me);
    give_me
}

/// This is kind of inconvenient though, so you can instead let the function reference the String
/// through a pointer ("borrowing" the String).
fn borrow_string(borrowed: &String) {
    println!("{}", *borrowed);

    // Dereferencing is implied, so it can be omitted:
    println!("{}", borrowed);
}

/// The annotation allows it to be logged into the console.
#[derive(Debug)]
struct StringHaver {
    possession: String,
}

/// Ownership of the String is tossed into the function and tossed back out inside a struct.
fn make_something_that_uses_string(donation: String) -> StringHaver {
    StringHaver {
        possession: donation,
    }
}

/// Without `'a` (the lifetime specifier/parameter), the compiler gets angry and says "expected
/// named lifetime parameter."
#[derive(Debug)]
struct StringPointerHaver<'a> {
    possession: &'a String,
}

/// This function must take a pointer that is created outside. Otherwise, when it then owns the
/// String and creates a pointer for it, the String will die when the function ends, so the pointer
/// in the `StringPointerHaver` would point to nothing. Nonideal! Not even a lifetime parameter
/// seems to be able to help.
fn make_something_that_uses_string_pointer(borrowed: &String) -> StringPointerHaver {
    StringPointerHaver {
        possession: borrowed,
    }
}

pub fn main() {
    let a = String::from("String to give to Exhibit A.");
    let_me_own_the_string(a);
    // println!("{}", a); <- Compile error because "value borrowed here after move"

    let b = String::from("String to give to Exhibit B.");
    let b2 = i_will_own_the_string_and_use_it_then_return_it(b);
    println!("{}", b2); // Ownership has been given back as b2 (b is still unusable though)

    // Can use a `let mut` to avoid creating a new variable/binding
    let mut c = String::from("String to give to Exhibit C.");
    c = i_will_own_the_string_and_use_it_then_return_it(c);
    println!("{}", c);

    // There is no ownership change here; `main` always owns `d`
    let d = String::from("String to give to Exhibit D.");
    borrow_string(&d);
    println!("{}", d);

    let e = String::from("String to give to Exhibit E.");
    let e2 = make_something_that_uses_string(e); // `e` is no longer usable.
    println!("{:?}", e2); // `{:?}` to format it with the derive(Debug) thing

    let f = String::from("String to give to Exhibit F.");
    let f2 = make_something_that_uses_string_pointer(&f);
    println!("{:?}", f2);
    println!("{}", f); // `f` is still usable because it was borrowed.

    let str_a: &str = "string";
    let str_b: &str = "string";
    let string_c: String = String::from("string");
    let string_d: String = String::from("string");
    println!("a == b is {}", str_a == str_b);
    println!("a == c is {}", str_a == string_c);
    println!("d == c is {}", string_d == string_c);
}
