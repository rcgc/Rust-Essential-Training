/*
    iterate over each item in a collection
    repeat a blocjk of code N times
 */
fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);

        if item == 'e'{
            println!("");
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}
