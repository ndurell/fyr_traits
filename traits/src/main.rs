// How to create a new Trait

// Traits can be used to group common functionality together.
trait Summary {
    fn summarize(&self) ->  &'static str;
}

struct Person {
    name: &'static str,
}

// To implement the Summary trait for Person we do the following:
impl Summary for Person {
    fn summarize(&self) ->  &'static str {
        self.name
    }
}

// Traits can provide default implementations of methods
trait Animal {
    fn make_noise(&self) {
        println!("Derpy derp!")
    }
}

struct Pangolin {
    _name: &'static str
}

// Empty implementation since we get make_noise for free.
impl Animal for Pangolin {}

struct Cow {
    _name: &'static str
}

// For cow we want to override the noise.
impl Animal for Cow {
    fn make_noise(&self) {
        println!("Dairy is scary!")
    }
}

// Traits can be implemented on a foreign type (a primitive type, even)
impl Summary for i32 {
    fn summarize(&self) ->  &'static str {
        fmt!("I'm an i32: {}!", self)
    }
}

fn main() {
    // Access new method using . syntax
    let eric = Person { name: "Eric" };
    println!("{}", eric.summarize()); 

    // Accessing the default implementation.
    let pangy = Pangolin { _name: "Pangy Pangerson" };
    pangy.make_noise();

    let clara = Cow { _name: "Clara" };
    clara.make_noise();


}
