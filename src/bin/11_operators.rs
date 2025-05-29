#![allow(unused)]


fn main() {
    let a: i32 =5;
    let b: i32 = 10;
    let c:i32 = a+b;
    println!("a: {}, b: {}, c: {}", a, b, c);

    //remainder and mod operators are not the same. 
    let d: i32 = 10 % 3; // remainder
    let e: i32 = 10 / 3; // integer division

    println!("d: {}, e: {}", d, e);


    //literals 
    let a =1i32;
    let b = 3u64;
    let c = 1.23e3;
    let d = 1_000_000_000u32; // underscores for readability
    
    
    //boolean operators
    let t = true;
    let f = false;
    let and = t && f; // logical AND
    let or = t || f; // logical OR
    let not = !t; // logical NOT



    //bitwise operators
    let a: u8 =5;
    let b: u8 = 3;
    let and = a & b; // bitwise AND
    let or = a | b; // bitwise OR
    let xor = a ^ b; // bitwise XOR
    let not = !a; // bitwise NOT
    let left_shift = a << 1; // left shift
    let right_shift = a >> 1; // right shift

    println!("a: {}, b: {}, and: {:03b}, or: {}, xor: {}, not: {}, left_shift: {}, right_shift: {}", 
             a, b, and, or, xor, not, left_shift, right_shift);
}