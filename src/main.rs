#![allow(dead_code)]
extern crate core;

use core::panic::PanicMessage;

mod slice_type;
mod user;

#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

enum Events {
    Wedding,
    Birthday,
    Point,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}
fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    width * height
}

fn square(point: &Point, side: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + side,
            y: point.y + side,
        },
    }
}
fn inspect(event: Events) {
    match event {
        Events::Wedding => println!("Wedding"),
        Events::Birthday => println!("Birthday"),
        Events::Point => println!("Point"),
    }
}
fn main() {}
