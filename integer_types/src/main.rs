/* Rust is a statically typed language
    * There are 4 basic data types to
    * represent individual scalar values
    * Integers from 8 to 128-bit
    * Floating point
    * Boolean
    * Character
*/

fn main() {
    let mut x: u8 = 255; 
    x = x + 1;  // data type overflow
    println!("x is {}", x);
}
