fn main() {
    let result = square(13);
    // result.0 contains x
    // result.1 contains x * x
    println!("result is {:?}", result);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    println!("End of function");
}