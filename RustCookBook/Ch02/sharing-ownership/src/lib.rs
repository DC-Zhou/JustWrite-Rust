#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

///
/// A length function that takes ownership of the input
/// variable
///
fn length(s:String) -> usize{
    s.len()
}

///
/// the same length function, taking ownership of a Rc
///
use std::rc::Rc;
fn rc_length(s: Rc<String>) -> usize{
    s.len()     // Calls to the wrapped object require no additions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloning() {
        let s = "abcdef".to_owned();
        assert_eq!(length(s), 6);
        // s is now "gone", we cant use it anymore
        // therefore we cant use it in a loop either
        // ... unless we clone s - at a const !
        let s  = "abcdef".to_owned();

        for _ in 0..10{
            // clone is typically an expensive deep copy
            assert_eq!(length(s.clone()), 6);
        }
    }

    #[test]
    fn refcounting(){
        let s = Rc::new("adcdef".to_owned());
        // we can clone RC (reference counters) with low cost
        assert_eq!(rc_length(s.clone()), 6);

        for _ in 0..10{
            assert_eq!(rc_length(s.clone()), 6);
        }
    }

    #[bench]
    fn bench_string_clone(b: &mut Bencher){
        let s: String = (0..100_000).map(|_|'a').collect();
        b.iter(|| { black_box(length(s.clone()));
        });
    }

    #[bench]
    fn bench_string_rc(b: &mut Bencher){
        let s: String = (0..100_00).map(|_| 'a').collect();
        let rc_s = Rc::new(s);
        b.iter(|| {
           black_box(rc_length(rc_s.clone()));
        });
    }

}
