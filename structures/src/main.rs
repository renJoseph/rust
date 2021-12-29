#[derive(Debug)]
struct Person {
    _name: String,
    _age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    _y: f32,
}

struct Rectangle {
    _top_left: Point,
    _bottom_right: Point,
}

fn main() {
    let _name = String::from("Ren");
    let _age = 27;

    let me = Person { _name, _age };

    println!("{:?}", me);

    let _bottom_right = Point { x: 1.0, _y: 2.0 };

    let _rectangle = Rectangle {
        _top_left: Point { x: 1.0, _y: 2.0 },
        _bottom_right,
    };

    println!("{}",_rectangle._bottom_right.x);

    let _unit = Unit;

    let _pair = Pair(1, 0.1);
}
