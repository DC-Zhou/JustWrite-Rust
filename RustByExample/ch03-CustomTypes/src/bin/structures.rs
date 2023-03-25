
#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Point{
    x: f32,
    y: f32,
}

struct Rectangle{
    p1: Point,
    p2: Point,
}

struct Unit;

struct Pair(i32, f32);

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle{p1: Point{x: x1, y: y1}, p2: Point{x: x2, y: y2}} = rect;
    ((x1 - x2) * (y1 - y2)).abs()
}

fn square(p: Point, l: f32) -> Rectangle {
    Rectangle{
        p1: Point{x:p.x, y: p.y},
        p2: Point{x: p.x + l, y: p.y + l},
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Rectangle{p1: Point{x: x1, y: y1}, p2: Point{x: x2, y: y2}} = *self;
        write!(f, "Rectangle: ({}, {}) -> ({}, {})", x1, y1, x2, y2)
    }
}

fn main() {
    // Create a structure with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate 'Point'
    let point : Point = Point{x: 0.3, y: 0.4};

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point{x: 0.1, ..point};

    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a 'let' binding
    let Point{x: my_x, y: my_y} = point;

    let _retangle = Rectangle{
        p1: Point{x: my_x, y: my_y},
        p2: point,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // sizeof struct
    println!("size of Person: {}", std::mem::size_of::<Person>());

    // sizeof rectangle
    println!("size of Point: {}", std::mem::size_of::<Point>());
    println!("size of Rectangle: {}", std::mem::size_of::<Rectangle>());

    // area of rectangle
    let rect = Rectangle{
        p1: Point{x: 0.0, y: 0.0},
        p2: Point{x: 3.0, y: 4.0},
    };

    println!("area of rectangle: {}", rect_area(rect));

    let p1 = Point{x: 1.0, y: 1.0};

    println!("square of rectangle: {}", square(p1, 2.2));



}