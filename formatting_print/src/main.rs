fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    //println!("c is {:.3}", c);
    //println!("c is {:08.3}\na is {}", c, a); // 08 means leading zeroes and 8 character space the output will occupy
    println!("c is {0:08.3}\na is {1}", c, a); // params have an index c is 0, a is 1 
}