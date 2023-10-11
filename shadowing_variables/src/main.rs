/*
    planet variable is redeclared when using let,
    however, if remove let it will produce an
    error because we are modifying an unmutable
    variable
*/
fn main() {
    let planet = "Earth";
    println!("planet is {}", planet);
    let planet = "Mars";
    println!("planet is {}", planet);
}
