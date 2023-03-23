fn main() {
    // scalar types
    let logical:bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    let default_float = 3.0; // `f64`
    let default_integer = 7;

    let mut inferred_type = 12; // type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable = 12; // mutable i32
    mutable = 21;

    // shadowing a variable
    let mutable = true; // mutable is now a boolean
}
