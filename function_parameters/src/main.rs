/*
    statement: performs an action without
                returning a value

                let sum = a + b;



    expression: evaluates to a resulting value,
                does not end with a semicolon
                a + b
                1 + 2
 */
fn main() {
    say_hello();
    say_a_number(13);
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello(){
    println!("Hello!");
}

fn say_a_number(number: i32){
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8){
    let sum = a + b;
    println!("sum is {}", sum);
}