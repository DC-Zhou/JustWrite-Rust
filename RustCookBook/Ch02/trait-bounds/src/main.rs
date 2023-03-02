use std::fmt::Debug;

///
/// a simple print function for printing debug formatted variables
///

fn log_debug<T: Debug>(t: T){
    println!("{:?}", t);
}

///
/// An interface that can be used for quick and easy logging
///
pub trait Loggable: Debug + Sized {
    fn log(self){
        println!("{:?}", &self)
    }
}

#[derive(Debug)]
struct ArbitaryType{
    v: Vec<i32>
}

impl ArbitaryType{
    pub fn new() -> ArbitaryType{
        ArbitaryType{
            v: vec![1,2,3,4]
        }
    }
}

impl Loggable for ArbitaryType { }

fn main() {
    let a = ArbitaryType::new();
    a.log();
    let b = 2;
    log_debug(b);
}
