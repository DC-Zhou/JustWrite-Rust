fn main() {
    // A counter variable
    let mut n = 1;

    // loop while 'n' is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizz buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }


    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100{
        if n % 15 == 0 {
            println!("fizz buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

}