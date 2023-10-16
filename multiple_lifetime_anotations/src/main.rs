/*
    Lifetime elision rules:
        1) Each input parameter that is a
        reference is assigned its own lifetime

        2) If there is exactly one input lifetime,
        assign it to all output lifetimes

        3) If there is a &self or &mut self
        input parameter, its lifetime will be 
        assigned to all output lifetimes
 */
fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}
