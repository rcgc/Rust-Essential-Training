fn main() {
    // let variables are immutable
    // by default Rust variables are immutable
    // mut indicates that it can be mutable

    // naming conventions: https://rust-lang.github.io/api-guidelines/naming.html

    let mut x = 10;
    
    println!("x is {}", x);

    x = 20;

    println!("x is {}", x);
}