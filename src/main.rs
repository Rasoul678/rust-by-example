#![allow(dead_code)]
use std::fmt::{self, Binary, Display, Formatter};
use to_binary::BinaryString;

fn main() {
    // This is a line comment
    println!("Hello, world!");

    let x = 5 +/* 90 + */ 5;
    println!("X is: {x}");

    // Formatted print
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // Justify text
    println!("{number:>5}", number = 1);

    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 10);

    // Debug
    struct Unprintable(i32);
    let up = Unprintable(10);
    // dbg!(up);

    #[derive(Debug)]
    struct DebugPrintable(i32);
    let dp = DebugPrintable(20);

    println!("{:?}", dp);

    dbg!(dp);

    let name = "Ghazal";
    let age = 27;

    let ghazal = Person { name, age };

    println!("{:?}", ghazal);

    // Display

    let number = Structure(12);

    println!("Number is: {}", number);

    let min_max = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // println!("What does Point2D look like in binary: {:b}?", point);
    println!("What does Ghazal look like in binary: {:b}?", ghazal);

    let v = List(vec![1, 2, 3]);
    println!("Display: {}", v);
    println!("Debug: {:?}", v);

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }

    for farbe in [
        Farbe {
            rot: 128,
            grün: 255,
            blau: 90,
        },
        Farbe {
            rot: 0,
            grün: 3,
            blau: 254,
        },
        Farbe {
            rot: 0,
            grün: 0,
            blau: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("Display: {}", farbe);
        println!("Debug: {:?}", farbe);
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl Binary for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let bin_str = BinaryString::from(self.name);

        write!(f, "{}", bin_str)
    }
}

#[derive(Debug)]
struct Structure(i32);

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3} {} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Farbe {
    rot: u8,
    grün: u8,
    blau: u8,
}

impl Display for Farbe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut r_hex = format!("{:X}", self.rot);
        let mut g_hex = format!("{:X}", self.grün);
        let mut b_hex = format!("{:X}", self.blau);

        if r_hex.len() == 1 {
            r_hex = format!("{:0>2}", r_hex);
        }
        if g_hex.len() == 1 {
            g_hex = format!("{:0>2}", g_hex);
        }
        if b_hex.len() == 1 {
            b_hex = format!("{:0>2}", b_hex);
        }

        let hex = format!("0x{r_hex}{g_hex}{b_hex}");
        write!(f, "RGB ({}, {}, {}) {hex}", self.rot, self.grün, self.blau)
    }
}

