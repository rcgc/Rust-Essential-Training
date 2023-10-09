/*
    Group multiple items of mixed data types

    Elements are ordered

    Stored in a fixed-length, contiguous section memory

    Data types of items must be known at compile time
*/
fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}
