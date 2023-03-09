fn to_reverse(a:&str) -> String{
    a.chars().rev().collect()
}


fn main(){
    println!("Hello Rust World");
    let str1 = String::from("Hello World");
    let mut str2 = to_reverse(&str1);
    println!("After to reverse: {}",str2);


    let str3 = str2.pop();

    match str3 {
        Some(d) => println!("It have char d !"),
        Some(o) => println!("It have char o !"),
        _ => println!("..."),
    }



}