enum WebEvent {
    Load,
    Destroy,
    KeyDown(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

type Operations = WebEvent;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::Load => println!("Page loaded"),
        WebEvent::Destroy => println!("Page destroyed"),
        WebEvent::KeyDown(c) => println!("Pressed '{}',", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("Clicked coordinates [ {} , {} ]", x, y)
        }
    }
}

enum VeryVerboseThingToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseThingToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Number {
    Zero,
    One,
    Two,
}

enum Colour {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let load = WebEvent::Load;
    let destroy = Operations::Destroy;

    use crate::WebEvent::{KeyDown};

    let key_down = KeyDown('c');
    let paste = WebEvent::Paste("string".to_owned());
    let click = WebEvent::Click { x: 100, y: 100 };

    inspect(load);
    inspect(destroy);
    inspect(key_down);
    inspect(paste);
    inspect(click);

    let add_result = VeryVerboseThingToDoWithNumbers::Add.run(12, 12);
    let subtract_result = VeryVerboseThingToDoWithNumbers::Subtract.run(12, 12);

    println!("{}", add_result);
    println!("{}", subtract_result);

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Colour::Red as i32);
    println!("violets are #{:06x}", Colour::Blue as i32);
}
