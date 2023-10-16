/*
    Collecion of elements with the same data type

    Elements are stored in order

    Items can be dynamically added and removed

    Stored in heap memory
*/
fn main() {
    let mut astronauts: Vec<String> = Vec::new();

    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));

    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    // let third = &astronauts[2];
    let third = astronauts.get(2);
    println!("third is {:?}", third);

    let countdown = vec![5, 4, 3, 2, 1];
}
