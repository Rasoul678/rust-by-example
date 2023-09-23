#![allow(unused_variables)]
use std::fmt::Display;

fn main() {
    // Conversion
    // From and Into
    let my_str = "hello";
    let my_string = String::from(my_str);

    // let num = Number::from(30);
    // println!("Number {{ {} }}", num);

    // `Into` trait is the reciprocal of `From` trait.

    let int = 5;
    let num: Number = 5.into();
    println!("Number {{ {} }}", num);

    // TryFrom and TryInto

    // TryFrom
    // assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    // assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // To and from Strings
    let circle = Circle {
        radius: 6,
        center: Center(10, 20),
    };
    
    println!("{}", circle.to_string());

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

struct Number {
    value: i32,
}

// impl From<i32> for Number {
//     fn from(value: i32) -> Self {
//         Number { value }
//     }
// }

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value: {}", self.value)
    }
}

#[derive(PartialEq, Debug)]
struct EvenNumber(i32);

// impl TryFrom<i32> for EvenNumber {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         match value {
//             v if v % 2 == 0 => Ok(EvenNumber(v)),
//             _ => Err(()),
//         }
//     }
// }

impl TryInto<EvenNumber> for i32 {
    type Error = ();

    fn try_into(self) -> Result<EvenNumber, Self::Error> {
        if self % 2 == 0 {
            Ok(EvenNumber(self))
        } else {
            Err(())
        }
    }
}

struct Center(i32, i32);

struct Circle {
    radius: i32,
    center: Center,
}

impl Display for Center {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circle of radius {}, and center of {}",
            self.radius, self.center
        )
    }
}
