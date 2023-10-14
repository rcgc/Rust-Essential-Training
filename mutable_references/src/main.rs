/*
    When using a mutable reference, you cannot
    create other references

    Prevents data reaces

    CORRECT:
    let ref1 = &mut var;

    CORRECT:
    let ref1 = &var;
    let ref2 = &var;

    INCORRECT:
    let ref1 = &mut var;
    let ref2 = &var;
*/

fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("prcoesing propellant {}...", propellant);
    propellant.push_str(" is highly flamable!");
    let length = propellant.len();
    length
}
