/*
    Ownership:

    Every variable is responsible of one and
    only one value

    Developer is responsible of free memory
    when requested in order to avoid overflow

    All of the following operations apply to
    strings (clone), because data is stored
    in heap. Cloning must be done explicitly 

    For integers and floats apply the following
    operations (copy) because data is stored in
    stack. Copy occurs implicitly
*/
fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone(); // duplicate on heap
        inner_planet.clear();
        println!("inner_planet is {}", inner_planet);
    }
    println!("inner_planet is {}", outer_planet);
}
