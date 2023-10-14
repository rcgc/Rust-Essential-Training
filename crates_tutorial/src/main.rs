/*
    Crates are a collection of rust source code
    files

    2 flavors:
        binary crate
        Libraries
*/
use rand::prelude::*;

fn main() {
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}

