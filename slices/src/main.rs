/*
    Reference to a contiguous section of a collection

    Commonly encountered as the string slice data type:&str

    String literals are slices

    Length is in bytes

    Range indices must occur at valid UTF-8
    character boundaries

    &String != &str
*/
fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    // let last_word = &message[15..15+5];
    let last_word = &message[15..];
    println!("last_word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    let inner_planets : &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets);

    let first_word = get_first_word(&message[10..]);
    println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..index]; //found a space!
        }
    }

    &s // no spaces found; input is a single word
}
