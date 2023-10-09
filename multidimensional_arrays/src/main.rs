fn main() {
    // You can have any number of sets
    // but they must be the same size
    let parking_lot = [[1, 2, 3],
                        [4, 5, 6]];
    
    let number = parking_lot[0][1];
    println!("number is {}", number);

    // A 5x20x100 garage matrix full of zeros
    let garage = [[[0; 100]; 20]; 5];
}
