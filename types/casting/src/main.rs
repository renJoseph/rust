#![allow(overflowing_literals)]

fn main() {
    let decimal: f32 = 65.4321;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);

    println!("1000 as u8 is: {}", 1000 as u8);
    // Same as
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!("-1 as u8 is: {}", -1i8 as u8);
}
