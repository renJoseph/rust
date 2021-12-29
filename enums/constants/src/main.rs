// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("Language: {}", LANGUAGE);
    println!("Threshold: {}", THRESHOLD);

    let number: i32 = 12;

    println!("is_big for number {}: {}", number, is_big(number));
}
