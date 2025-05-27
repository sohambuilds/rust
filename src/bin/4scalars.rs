#![allow(unused_variables)]

fn main() {

    let i0: i8 = 127; // 8-bit signed integer
    let i1: i16 = 32767; // 16-bit signed integer
    let i2: i32 = 2147483647; // 32-bit signed integer
    let i3: i64 = 9223372036854775807; // 64-bit signed integer
    let i4: i128 = 170141183460469231731687303715884105727; // 128-bit signed integer   

    let i5: isize = 100000000000000;


    let u0: u8 = 255;
    let u1:u16 = 65535;
    let u2: u32 = 4294967295;
    let u3: u64 = 18446744073709551615;
    let u4: u128 = 3402823668417103009491288315;
    let u5: usize = 100;

    let f0: f32 = 3.14; // 32-bit floating point
    let f1: f64 = 3.141592653589793; // 64-bit floating point

    let b: bool = true; // boolean type
    let c: char = 'a'; // character type
  

    let i: i32 = 1;
    let u: u32 = i as u32; // casting from signed to unsigned

    let x: u32 = u + (i as u32); // adding signed and unsigned integers


    //Min and Max values

    let min_i: i32 = i32::MIN; // minimum value of i32
    let max_i: i32 = i32::MAX; // maximum value of i32
    let min_u: u32 = u32::MIN; // minimum value of u32      
    println!("Min i32: {}, Max i32: {}", min_i, max_i);  
    println!("Min u32: {}", min_u);   


    let mut u: u32 = u32::MAX; // maximum value of u32
    u += 1; // overflow occurs here 
    println!("Overflowed u32: {}", u); // this will panic at runtime in debug mode

  
    let u = u32::checked_add(u32::MAX,1); // checked addition that returns an Option
   println!("Checked addition result: {:?}", u); // this will print None if overflow occurs

    let u = u32::wrapping_add(u32::MAX, 1); // wrapping addition that wraps around on overflow
    println!("Wrapping addition result: {}", u); // this will wrap around to 0 if overflow occurs

  

}
  