// Rust is a low level language
// meaning we must interact with the hardware
// hardware are not safe
// so rust enforce memory safety
// hardware is not safe, so rust prevent use from doing certain task
// we must write unsafe code to perform certain task
// usafe kwyword to write unsafe code
// 5 FEATURES OF UNSAFE CODE
/*
    Dereference a raw pointer
    Call an unsafe function or method
    Access or modify a mutable static variable
    Implement an unsafe trait
    Access fields of unions

*/

// Calling an Unsafe Function or Method
// Creating a Safe Abstraction over Unsafe Code
// just because a function contains unsafe code doesn’t mean we need to mark the entire
// function as unsafe.
//
//
use std::slice;

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using extern Functions to Call External code
extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn unsave() {
    // Dereferencing a Raw pointer
    // raw pointers can be immutable or mutable and are written as *const T and *mut T
    // the asterisk isn’t the dereference operator; it’s part of the type name.
    //
    /* Different from references and smart pointers, raw pointers:

    Are allowed to ignore the borrowing rules by having both immutable and mutable pointer
    or multiple mutable pointers to the same location
    Aren’t guaranteed to point to valid memory
    Are allowed to be null
    Don’t implement any automatic cleanup
    */
    //
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {} and r2 is {:?}", *r1, *r2);
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
