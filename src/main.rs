#![allow(unused_variables)]
#![allow(unused_assignments)]
use std::fmt::{self, Debug, Display, Formatter};
use std::mem;

fn main() {
    let logical = true;

    let a_float: f64 = 1.5; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    let default_float = 1.1;
    let default_integer = 6;

    let mut inferred_type = 12;
    inferred_type = 60i64;

    let mut mutable = 12;
    mutable = 67;

    // Error
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    // Literals and operators
    let e_notation = 1e6;
    println!("{}", e_notation);

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Tuples
    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,12,13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    // Arrays and Slices
    // Fixed-size array (type signature is superfluous).
    let xs = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    let str_array = ["Ghazal","Rasoul"];
    analyze_slice(&str_array);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(x_val) => println!("{}: {}", i, x_val),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    // println!("{}", xs[..][5]);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} ) \n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(mx: Matrix) -> Matrix {
    Matrix(mx.0, mx.2, mx.1, mx.3)
}

// This function borrows a slice.
fn analyze_slice<T: Debug>(slice: &[T]) {
    println!("First element of the slice: {:?}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
