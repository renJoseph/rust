fn main() {
    let _is_true: bool = true;

    let _a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // otherwise defaults are used
    let _default_float_i64 = 3.0;
    let _default_integer_i32 = 7;

    // type can also be inferred from context
    let mut _inferred_type = 12;
    _inferred_type = 4294967295i64;

    // a mutable variable's value can be changed
    let mut _mutable = 12;
    _mutable = 10;

    // _mutable = true; Error! Can't change type

    // variables can be overwritten with shadowing
    let _mutable = true;
}
