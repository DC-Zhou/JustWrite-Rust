
// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age:u8
}

fn print_struct(){
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    // Pretty print
    println!("{:#?}", peter);
}
fn main() {

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
    "Slater",
    "Christian",
    actor = "actor's");

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    print_struct();
}