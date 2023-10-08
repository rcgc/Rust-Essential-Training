fn main() {
    //let value = 0b1111_0101; // will be stored as an integer and by default will use the signed 32-bit integer data type
    let mut value = 0b1111_0101u8; // unsigned 8bit
    println!("value is {}", value);
    println!("value is {:08b}", value); // colon indicates especial format, b indicates binary bits 8 number of bits to display, 0 is for leading zeroes

    // NOT
    value = !value;
    println!("value is {:08b}", value);

    // AND
    value = value & 0b1111_0111;
    println!("value is {:08b}", value);

    // Check specific bit position
    println!("bit 6 is {}", value & 0b0100_0000);

    // OR
    value = value | 0b0100_0000;
    println!("value us {:08b}", value);

    // XOR
    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value);

    // LEFT SHIFT
    value = value << 4;
    println!("value is {:08b}", value);

    // RIGHT SHIFT
    value = value >> 2;
    println!("value is {:08b}", value);
}
