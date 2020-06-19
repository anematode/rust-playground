/// Baila con su jugadora favorita.
///
/// # Ejemplo
///
/// ```
/// let mut manzana = 3;
/// manzana = 4;
/// let sigue = "a gamepro5";
/// ```
pub fn gutchen() -> i8 {
    println!("{}", years_of_age::intro_ducir());
    -3
}

fn privadaaaaa() -> String {
    String::from("sity")
}

mod years_of_age {
    struct Thirt {
        y_one: i64,
        six_six: String,
    }

    /// Perro.
    fn sixty() -> Thirt {
        return Thirt {y_one: 31, six_six: String::from("66")}
    }

    pub fn intro_ducir() -> String {
        let eight = sixty();
        println!("{}", super::privadaaaaa());
        String::from(format!("thirt{}...{}", eight.y_one, eight.six_six))
    }
}
