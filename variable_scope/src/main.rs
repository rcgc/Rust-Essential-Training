/*
    Second println statement cannot access
    variable scope
*/
fn main() {
    if true {
        let planet = "Earth";
        println!("planet is {}", planet);
    }
    // println!("planet is {}", planet);
}
