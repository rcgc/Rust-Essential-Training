/*
    loop can be used to return a value
    On the other "while" not
*/

fn main() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}
