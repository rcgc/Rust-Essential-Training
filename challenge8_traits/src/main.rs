/*
    Trait:
        - A collecion of methods
        - Data types can implement a trait
        - Generics use traits to specify the
        capabilities of unknown data types
        - Similar to interfaces in other
        programming languages

        - It is a contract that specifies
        certain capabilities that a type
        must have
        - This allows you to create abstractions
        and generics

    Derivable Traits:
        - Eq
        - PartialEq: Only if all the fields are equal
        - Ord
        - PartialOrd
        - Clone
        - Copy
        - Hash
        - Default
        - Debug

    Trait bounds
        - Bounding restrics a generic to only data 
        types that conform those bounds

    where
        - Makes a function signature more readable
        by grouping together multiple traits bounds.
        This can be especially helpful when there are
        many input variables and trait bounds
*/
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

/* YOUR CODE GOES HERE */
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} flying at {} miles per hour", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
