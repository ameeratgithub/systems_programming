#![allow(dead_code)]
fn main() {
    one_type_to_another();
}

///
/// This example illustrates that similar bits can represent different data or different
/// meaning in different contexts.
/// It's the type system of the language which makes this distinction, CPU doesn't know anything
/// about it.
///
fn data_type_determines_what_bits_represent() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {a:016b} {a}");
    println!("b: {b:016b} {b}");

    assert_eq!(format!("{a:016b}"), format!("{b:016b}"));
}

///
/// This function demonstrates that we can cast one type to another via `std::mem::transmute`
/// This is exteremly unsafe operation. There are less dangerous operations depending on the
/// context. Use this function as a last resort.
///
fn one_type_to_another() {
    let a: f32 = 42.42;
    // `transmute` reinterprets raw bits of f32 as a value of u32
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{frankentype}");
    println!("{frankentype:032b}");

    // `transmute` reinterprets raw bits of u32 as raw bits of f32
    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{b}");
    assert_eq!(a, b);
}
