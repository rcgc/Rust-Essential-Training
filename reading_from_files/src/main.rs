use::std::fs;

fn main() {
    let path = "C:/Users/RobertoCarlos/Documents/Rust-Essential-Training/reading_from_files/src/planets.txt";

    let contents = fs::read_to_string(path).unwrap();
    println!("\ncontents is {}\n", contents);

    for line in contents.lines(){
        println!("line is {}", line);
    }

    // As a vector
    let contents = fs::read(path).unwrap();
    println!("\ncontents is {:?}\n", contents);
}
