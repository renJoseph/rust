use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn main() {
    let num = Number::from(30);

    println!("My number is {:?}", num.value);

    let num2: Number = 30.into();

    println!("My number is {:?}", num2);
}
