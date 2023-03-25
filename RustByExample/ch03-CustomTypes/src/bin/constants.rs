static LANGUAGE: &str = "Rust";
static THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn coerce_static(_: &i32) -> &i32 {
    &THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    {
        let lifetime_num = 9;

        let _coerce_static = coerce_static(&lifetime_num);

        println!("_coerce_static: {}", _coerce_static);
    }
}