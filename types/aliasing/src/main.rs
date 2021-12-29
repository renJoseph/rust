type NanoSecond = u64;
type Inch = u64;

fn main() {
    let nanoseconds: NanoSecond = 5;
    let inches = nanoseconds as Inch;

    println!("{}", inches);
}
