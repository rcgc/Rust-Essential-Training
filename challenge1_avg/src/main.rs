fn main() {
    // default integer value is i32
    // default floating-point value is f64

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let average = (a as f64 + b + c as f64) / 3.0;
    // f32 will produce 45.100002

    assert_eq!(average, 45.1);
    println!("Test passed!");
}