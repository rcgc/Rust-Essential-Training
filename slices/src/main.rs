/*
    Reference to a contiguous section of a collection

    Commonly encountered as the string slice data type:&str

    String literals are slices

    Length is in bytes

    Range indices must occur at valid UTF-8
    character boundaries
*/
fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    // let last_word = &message[15..15+5];
    let last_word = &message[15..];
    println!("last_word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    let inner_planets : &[i32] = &planets[..4];
    print!("inner_planets are {:?}", inner_planets);
}
