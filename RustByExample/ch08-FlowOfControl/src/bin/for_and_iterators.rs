fn main() {
    // use iter()
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a crustacean among us !"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    let names_1 = vec!["Bob", "Frank", "Ferris"];

    // use into_iter()
    for name in names_1.into_iter() {
        match name {
            "Ferris" => println!("There is a crustacean among us !"),
            _ => println!("Hello {}", name),
        }
    }


    // use iter_mut
    let mut names_2 = vec!["Bob", "Frank", "Ferris"];

    for name in names_2.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us !",
            _ => "Hello ",
        }
    }

    println!("names: {:?}", names);
}