use std::fmt;

fn main() {

    // {} replace by args
    println!("{} days", 31);
    // {} replace also marked args layer
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

    println!("{object}  {verb}  {subject}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb    = "jumps over");

    // Different formatting can be invoked by specifying the format :
    println!("Base 10:              {}",   42);
    println!("Base 2(binary):       {:b}", 42);
    println!("Base 8:               {:o}", 42);
    println!("Base 16:              {:x}", 42);

    //
    println!("{number:>5}", number = 1);

    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}.", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // fmt::Display;
    // println!("This struct '{}' won't print...", Structure(3));


    let number:f64  = 1.0;
    let width: usize = 5;
    print!("{number:>width$}\n");


    // Activities
    let pi = 3.141592653;
    println!("pi = {}", pi);


}
