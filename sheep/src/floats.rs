pub fn main() {
    let one = 1.0;
    let zero = 0.0;
    let negative_zero = -0.0;
    println!(
        "{} is equal to itself? {}",
        zero / zero,
        zero / zero == zero / zero
    );
    println!("1.0 / -0.0 = {}", one / negative_zero);
}
