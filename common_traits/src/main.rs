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
struct NotDebug {}

#[derive(Debug)]
struct Cow {
    name: String,
    not_debuggable: NotDebug
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
// can be derived as well.
trait Clone : Sized {
    fn clone(&self) -> Self;
}

// The Copy trait
// This is a marker trait
// Notice though that to be a Copy means you are required to be a Clone.
trait Copy: Clone { }

// Copy has no methods to implement but it can be derived
// Indicates that the object can be copied by just moving the bits

// Must implement both Copy and Clone since clone is a super type.
#[derive(Debug,Copy,Clone)]
struct Person {
    age: i32
}

// Other cool marker traits Sync/Send.
// These traits help the compiler know a type is thread safe.

fn main() {
    // using ToString via Display
    let i: i32 = 5;
    println!("I like {}", i.to_string());

    // Display usage
    // Any type that implements Display automatically implements ToString
    println!("I like to display {}", i);

    // Debug usage
    let bessie = Cow { name: "Bessie".to_string(), not_debuggable: NotDebug {} };
    println!("DEBUG: {:?}", bessie);
    dbg!(bessie);

    // Default usage
    let point = Point::default();
    println!("point.x = {}", point.x);
    println!("point.y = {}", point.y);

    // From/Into usage
    let reg_str = "Eric";
    let from_str = String::from(reg_str);
    println!("Hi {}!", from_str);
    let mut into_str: String = from_str.into();
    println!("Hi again {}!", into_str);

    // Clone
    let clone_str = into_str.clone(); 
    into_str.push_str(" is cool!");
    println!("into_str {}", into_str); // prints into_str Eric is cool! 
    println!("clone_str {}", clone_str); // prints clone_str Eric

    let eric = Person { age: 26 };
    let eric_copy = eric;
    // eric_copy is a copy of eric.
    println!("eric {:?}", eric) // this is ok.

}
