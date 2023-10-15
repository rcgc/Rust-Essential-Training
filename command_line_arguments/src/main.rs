use::std::env;

/*
    cargo run Moon 1969 --flag

    a common practice is to collect all
    the arguments in vectors
*/
fn main() {
    if env::args().len() <= 2{
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate(){
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
