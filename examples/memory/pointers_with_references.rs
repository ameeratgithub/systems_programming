#![allow(dead_code)]

use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    invalid_pointer();
}

/// 
/// Raw pointers are not safe because of following properties, and their use is discouraged
/// with-in day-to-day Rust code.
/// 1. Raw pointers don't own their values. Rust compiler doesn't check that the referent
/// data is still valid when they are accessed.
/// 2. Multiple raw pointers to the same data are allowed. Every raw pointer can have write,
/// read-write access to the data. This means that there is no time when Rust can guarantee
/// that shared data is valid.
/// 
/// There are also valid reasons to use raw pointers.
/// 1. It's unavoidable. Perhaps some OS call or third party code requires a raw pointer.
/// 2. Shared access to something is essential and runtime performance is paramount. Perhaps
/// multi components within your application require equal access to some expensive-to-compute
/// variable.
/// 
fn invalid_pointer() {
    // You can safely create pointers from any integral value. An i32 is not a Vec<String>
    // but Rust is quite comfortable here.
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{ptr:p} -> {new_addr:p}");
    }
}

fn identify_value_address() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    // Casting a_ptr to usize. transmute is highly unsafe function, use it carefully
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {a} ({a_ptr:p}...0x{:x})", a_addr + 7);
}

fn printing_external_strings() {
    let a = 42;
    // String is a smart pointer type that holds a pointer to backing array and a field
    // to store its size.
    let b: String;
    // Cow means copy on write. This smart pointer is useful when an external source provides
    // a buffer
    let c: Cow<str>;

    unsafe {
        // References cannot be cast directly to *mut T, the type required by
        // String::from_raw_parts. But *const T can be cast to *mut T, leading to this
        // double cast syntax.
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        // Converts *const u8 to *const i8, aliased to c_char. The conversion to i8 works
        // because we remain under 128, following the ASCII standard.
        let c_ptr = &C as *const u8 as *const c_char;
        // Conceptually, CStr::from_ptr takes responsibility for reading the pointer until it
        // reaches 0; then it generates Cow<str> from the result
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {a}, b: {b}, c: {c}");
}

fn pointers_sizes() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location:   {:p}", &a);
    println!("  size:       {:?} bytes", size_of::<usize>());
    println!("  value:      {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location:   {:p}", &b);
    println!("  size:       {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to:  {:p}", b);
    println!();

    println!("c (a \"box\" for C):");
    println!("  location:   {:p}", &c);
    println!("  size:       {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to:  {:p}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location:    {:p}", &B);
    println!("  size:        {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:       {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location:    {:p}", &C);
    println!("  size:        {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:       {:?}", C);
    println!();
}
