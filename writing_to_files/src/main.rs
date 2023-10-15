use std::fs;
use std::io::prelude::*;

fn main() {
    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    let path_speech = "C:/Users/RobertoCarlos/Documents/Rust-Essential-Training/writing_to_files/src/speech.txt"; 

    fs::write(path_speech, speech);


    let path_planets = "C:/Users/RobertoCarlos/Documents/Rust-Essential-Training/reading_from_files/src/planets.txt";
    let mut file = fs::OpenOptions::new().append(true).open(path_planets).unwrap();
    file.write(b"\nPluto");

}
