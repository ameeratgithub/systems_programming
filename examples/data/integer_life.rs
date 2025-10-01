#![allow(dead_code)]
fn main() {
    // integer_overflow();
    // u16_bits();
    // arithmetic_overflow();
    endianess();
}

///
/// This function will panic in debug mode when `i` gets larger than `u16::MAX`
/// It will not panic in release mode, but will again start from zero, if i gets larger than
/// `u16::MAX`.  
///
/// So in debug mode, if `i` tries to add 1 to 65535, it will panic
/// In release mode, if `i` tries to add 1 to 65535, it will overflow and will again start
/// at 0
///
fn integer_overflow() {
    let mut i: u16 = 0;
    print!("{i}...");
    loop {
        i += 1000;
        print!("{i}...");
        if i % 10000 == 0 {
            println!();
        }
    }
}

///
/// This function demostrates bits representation of `u16` type
///
fn u16_bits() {
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{sixtyfivethousand_533}, {sixtyfivethousand_534}, {sixtyfivethousand_535}");
}

///
/// Simpler example of arithmetic overflow
/// In debug mode, this code will panic
/// In release mode (optimization enabled), it will result in '144', which is incorrect
///
fn arithmetic_overflow() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("{a} + {b} = {c}");
}

///
/// Simple demonstration of endianess of CPUs
/// Most computers will see the following output
/// -573785174 vs -1430532899
/// But some exotic ones can swap these values like this
/// -1430532899 vs -573785174
///
/// Interger are almost always stored in little endian format
///
fn endianess() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{a} vs {b}");
}
