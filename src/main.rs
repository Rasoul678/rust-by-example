// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::List::*;
use crate::Status::{Poor, Rich};
use crate::Work::*;

// Globals are declared outside all other scopes.
static mut LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    // Structures
    // Create struct with field init shorthand
    let name = String::from("Ghazal");
    let age = 27;
    let ghazal = Person { name, age };

    // Print debug struct
    println!("{:?}", ghazal);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    println!("{:?}", rectangle);

    // Instantiate a unit struct
    let unit = Unit;

    println!("{:?}", unit);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Enums

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("Ghazal".to_string());
    let clicked = WebEvent::Click { x: 10, y: 20 };
    let loaded = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;

    inspect(&loaded);
    inspect(&pressed);
    inspect(&pasted);
    inspect(&clicked);
    inspect(&unloaded);

    // Type aliases
    let x = Operations::Add;

    let added_val = Operations::Add.run(30, 12);
    println!("Add {}", added_val);

    let status = Poor;
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // C-like enums
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // Test case: linked-list
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // constants
    let n = 16;

    // Access constant in the main thread
    unsafe {
        println!("This is {}", LANGUAGE);
    }
    
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    unsafe {
        LANGUAGE = "Ghazal";
        println!("This is {}", LANGUAGE);
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
#[derive(Debug)]
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        bottom_right: Point { x: x1, y: y1 },
        top_left: Point { x: x2, y: y2 },
    } = rect;

    32.0
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded!"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'!", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y)
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}
