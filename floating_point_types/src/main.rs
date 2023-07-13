/*
    * Represent numbers with decimal points
    * using IEEE 754
    * Rust has two floating-point types: 
    * f32 and f64
    *
    * Value stored as fractional and 
    * exponential components
    * Max f64 value approx 
    * 1.7976931348623157 x 10^308
*/

fn main() {
    let x: f32 = 10.123456789123456789;
    println!("x is {}", x);
}
