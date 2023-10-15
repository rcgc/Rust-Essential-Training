use std::env;
use std::fs;

/*

    cargo run C:/Users/RobertoCarlos/Documents/Rust-Essential-Training/challenge5_check_the_roster/src/moonwalkers.txt Cernan

    cargo run C:/Users/RobertoCarlos/Documents/Rust-Essential-Training/challenge5_check_the_roster/src/moonwalkers.txt Stone
*/

fn main() {
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}
