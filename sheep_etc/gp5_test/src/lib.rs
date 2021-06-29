/// The Gamepro5 test, inefficiently concatenating a string an insane amount to
/// somehow prove that Java is less efficient than JavaScript.
///
/// ```java
/// public class Main {
///
///     public static String Repeat(String item,  repeat) {
///             String thing = item;
///             for (int i=0;i<repeat-1;++i) {
///             thing = thing + ", " + item;
///             };
///             return thing;
///         };
///
///     public static void main(String[] args) {
///             System.out.println(Repeat("hi people!", 100000));
///         };
///
///     };
/// ```
pub fn gp5_test(item: &String, repeat: usize) -> String {
    let mut thing: String = item.to_string();
    for _ in 0..(repeat - 1) {
        thing = thing + ", " + item;
    }
    thing
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::gp5_test(&format!("sheep"), 3), "sheep, sheep, sheep");
    }
}
