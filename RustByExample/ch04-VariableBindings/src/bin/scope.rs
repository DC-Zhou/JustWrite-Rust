fn main() {
    let long_lived_binding = 1;

    // This is a short lived binding
    let short_lived_binding = 1;

    // This is a block
    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

}