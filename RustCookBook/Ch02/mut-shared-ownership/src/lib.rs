#![feature(test)]

extern crate test;

use test::{ Bencher};

// before use cargo +nightly test, make sure delete your target file
// and then use cargo +nightly bench, checkout bench base

use std::cell::{Cell, RefCell};
use std::borrow::Cow;
use std::ptr::eq;

fn min_sum_cow(min: i32, v: &mut Cow<[i32]>) {
    let sum :i32 = v.iter().sum();
    if sum < min{
        v.to_mut().push(min - sum);
    }
}

fn min_sum_refcell(min: i32, v: &RefCell<Vec<i32>>){
    let sum: i32 = v.borrow().iter().sum();
    if sum < min{
        v.borrow_mut().push(min - sum);
    }
}

fn min_sum_cell(min: i32, v: &Cell<Vec<i32>>){
    let mut vec = v.take();
    let sum :i32 = vec.iter().sum();
    if sum < min {
        vec.push(min - sum);
    }
    v.set(vec);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn about_cells() {
        // we allocate memory and use a RefCell to dynamically manage ownership
        let ref_cell = RefCell::new(vec![10, 20, 30]);

        // mutable borrows are fine
        min_sum_refcell(70, &ref_cell);

        // they are equal!
        assert!(ref_cell.borrow().eq(&vec![10, 20, 30, 10]));

        // cells are a bit different
        let cell = Cell::from(vec![10, 20, 30]);

        // pass the immutable cell into the function
        min_sum_cell(70, &cell);

        // unwrap
        let v = cell.into_inner();

        // check the contents, and they changed!
        assert_eq!(v, vec![10, 20, 30, 10]);
    }

    #[test]
    #[should_panic]
    fn failing_cells(){
        let ref_cell = RefCell::new(vec![10, 20, 30]);

        // multiple borrows are fine
        let _v = ref_cell.borrow();
        min_sum_refcell(60, & ref_cell);

        // ... until the are mutable borrows
        min_sum_refcell(70, &ref_cell);
    }

    #[test]
    fn handling_cows() {
        let v = vec![10,20,30];

        let mut cow = Cow::from(&v);
        assert!(eq(&v[..], &*cow));

        min_sum_cow(70, &mut cow);

        assert_eq!(v, vec![10,20,30]);
        assert_eq!(cow, vec![10,20,30,10]);
        assert!(!eq(&v[..], &*cow));

        let v2 = cow.into_owned();
        let mut cow2 = Cow::from(&v2);
        min_sum_cow(70, &mut cow2);

        assert_eq!(cow2, v2);
        assert!(eq(&v2[..], &*cow2));
    }

    #[bench]
    fn bench_regular_push(b: &mut Bencher){
        let mut v = vec![];
        b.iter( || {
            for _ in 0..1_000{
                v.push(10);
            }
        });
    }

    #[bench]
    fn bench_refcell_push(b: &mut Bencher){
        let v = RefCell::new(vec![]);
        b.iter( || {
            for _ in 0..1_000{
                v.borrow_mut().push(10);
            }
        });
    }

    #[bench]
    fn bench_cell_push(b: &mut Bencher){
        let v = Cell::new(vec![]);
        b.iter( || {
            for _ in 0..1_000{
                let mut vec = v.take();
                vec.push(10);
                v.set(vec);
            }
        });
    }


}
