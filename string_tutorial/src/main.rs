/*
    String literal
    is different from
    String type
*/
fn main() {
    // message has been stored in heap
    // pointer to message are stored in stack
    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str("is home.");
    println!("message is {}", message);
}
