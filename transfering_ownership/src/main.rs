/* 
    To be an efficient rust developer
    it's necessary distinguish between
    stack and heap storage, allocation and
    how to switch between them
*/
fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String{
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}
