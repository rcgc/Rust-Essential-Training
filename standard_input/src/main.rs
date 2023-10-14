/*
    Rust standard library isn't as expensive 
    as in another programming languages

    io library isn't included in the prelude,
    so we need to import it
*/
use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a number");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    //let number = buffer.trim().parse::<i32>();
    let number: i32 = buffer.trim().parse().unwrap();
    println!("number +1 is {}", number + 1);
}