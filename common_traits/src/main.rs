use core::fmt::Error;
use std::fmt::Formatter;


// ToString trait
// Implemented on many built in types (like i32, String, etc...)
trait ToString {
    /// Converts the given value to a `String`.
    fn to_string(&self) -> String;
}

// Display trait
// Provides a way for an object to encoded in a string with {}
trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// Debug trait
// Similar to display except that it can be derived (we've seen this before.)
// Provides a way for an object to encode in a string with {:?}
trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

#[derive(Debug)]
struct Cow {
    name: String
}

// Default trait
// Provides sensible default values for struct members and can be derived.
// like zero for numbers, empty for vectors, “”
trait Default {
    fn default() -> Self;
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32
}

// The From and Into traits
// From allows a value to be converted from one type to another.
// When From is implemented you get Into automatically.
trait From<T> {
    fn from(_: T) -> Self;
}
// into can be called on an object to get a different type
trait Into<T> {
    fn into(self) -> T;
}

// The Clone trait
// provides a method to clone an object
trait Clone : Sized {
    fn clone(&self) -> Self;
}

#[derive(Clone)]
struct Person {
    age: i32
}

fn main() {
    // using ToString
    let i = 5;
    println!("I like {}", i.to_string());

    // Display usage
    // Any type that implements Display automatically implements ToString
    println!("I like to display {}", i);

    // Debug usage
    let bessie = Cow { name: "Bessie".to_string() };
    println!("DEBUG: {:?}", bessie);

    // Default usage
    let point = Point::default();
    println!("point.x = {}", point.x);
    println!("point.y = {}", point.y);

    // From/Into usage
    let reg_str = "Eric";
    let from_str = String::from(reg_str);
    println!("Hi {}!", from_str);
    let into_str: String = reg_str.into();
    println!("Hi again {}!", into_str)

}
