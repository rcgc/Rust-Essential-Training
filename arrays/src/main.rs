/*
    An array is a collection of values with the
    same data type

    Fixed length
*/

fn main() {
    let letters = ['a', 'b', 'c'];
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);
    
    let numbers: [i32; 5];
    //numbers = [0, 0, 0, 0, 0];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index-1]);
}
