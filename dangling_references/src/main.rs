fn main() {
    let rocket_fuel = produce_fuel();
    println!("rocket fuel is {}", rocket_fuel);
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
