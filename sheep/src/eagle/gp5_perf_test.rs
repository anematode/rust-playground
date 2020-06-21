//! From the words of the holy man Gamepro5 himself:
//!
//! > the java language is so slow
//! >
//! > [a code snippet, shown below]
//! >
//! > this took half a minute to process
//!
//! ```java
//! public class Main {
//!
//!     public static String Repeat(String item,  repeat) {
//!         String thing = item;
//!         for (int i=0;i<repeat-1;++i) {
//!             thing = thing + ", " + item;
//!         };
//!         return thing;
//!     };
//!
//!     public static void main(String[] args) {
//!         System.out.println(Repeat("hi people!", 100000));
//!     };
//!
//! };
//! ```

use std::time::Instant;

/// Creates a new string per iteration
fn repeat(item: String, repeat: i32) -> String {
    let mut thing = item.clone();
    for _ in 0..(repeat - 1) {
        thing = thing + ", " + item.as_str();
    }
    thing
}

pub fn main() {
    let now = Instant::now();
    println!("{}", repeat(String::from("hi people!"), 100000));
    println!("That took {}μs.", now.elapsed().as_micros());
}
