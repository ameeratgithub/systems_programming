#![allow(dead_code)]

static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    valid_addresses();
}

fn valid_addresses() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:       {:p}", &GLOBAL as *const i32);
    println!("local_str:    {:p}", local_str as *const str);
    println!("local_int:    {:p}", &local_int as *const i32);
    println!("boxed_int:    {:p}", Box::into_raw(boxed_int));
    println!("boxed_str:    {:p}", Box::into_raw(boxed_str));
    println!("fn_int:       {:p}", fn_int);
}

fn segmentation_fault() {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        // Converts i to a *const T, a raw pointer of type 8 to inspect raw memory address
        // We treat every address as a unit, ignoring the fact that most values span
        // multiple bytes
        let ptr = i as *const u8;
        // Read the value being pointed to
        // It will crash, if and when i = 0, it can't really be dereferenced. That's why unsafe block is used
        // If we start from 1, to avoid NULL pointer dereferencing, it will still crash. It's a segmentation
        // fault, which occures when the CPU and OS detect that program is attempting to access the memory
        // that it's not entitled to.
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {n_nonzero}");
}
