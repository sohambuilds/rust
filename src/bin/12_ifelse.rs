#![allow(unused)]

fn main(){

    let x: i32 = 10;

    if x%2 == 0 {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }


    let z: i32 = if x>0 {
        1
    } else if x<0 {

        -1
    } else {
        0
    };

    println!("z: {}", z);



}