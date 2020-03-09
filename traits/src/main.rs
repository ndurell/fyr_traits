// How to create a new Trait

// Traits can be used to group common functionality together.
trait Summary {
    fn summarize(&self) -> String;
}

struct Person {
    name: String,
}

// To implement the Summary trait for Person we do the following:
impl Summary for Person {
    fn summarize(&self) -> String {
        self.name.to_string()
    }
}

// Traits can provide default implementations of methods
trait Animal {
    fn make_noise(&self) {
        println!("Derpy derp!")
    }
}

struct Pangolin {
    _name: String
}

// Empty implementation since we get make_noise for free.
impl Animal for Pangolin {}

struct Cow {
    name: String
}

// For cow we want to override the noise.
impl Animal for Cow {
    fn make_noise(&self) {
        println!("Dairy is scary {}!", self.name)
    }
}

// Traits can be implemented on a foreign type (a primitive type, even)
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("I'm an i32: {}!", self)
    }
}

// Static vs Dynamic Dispatch

// We can consume a trait like this.
fn make_animal_noise<T: Animal>(animal: &T) {
    animal.make_noise()
}

// Dynamic dispatch
// The structure below would give us a vec of pointers to animal
// Calls to make_noise would use a vtable to lookup the method.
// This is different from the above where rust generates the code for each
// invocation of make_animal_noise.
struct Managerie {
    animals: Vec<Box<dyn Animal>>,
}

fn main() {
    // Access new method using . syntax
    let eric = Person { name: "Eric".to_string() };
    println!("{}", eric.summarize()); 

    // Accessing the default implementation.
    let pangy = Pangolin { _name: "Pangy Pangerson".to_string() };
    pangy.make_noise();

    let clara = Cow { name: "Clara".to_string() };
    clara.make_noise();

    let i: i32 = 32;
    println!("{}", i.summarize());

    // static dispatch: rust generates code for each type
    make_animal_noise(&pangy);
    make_animal_noise(&clara);

    let managerie = Managerie { animals: vec!(Box::new(pangy), Box::new(clara)) };
    managerie.animals[0].make_noise();
    managerie.animals[1].make_noise();

}
