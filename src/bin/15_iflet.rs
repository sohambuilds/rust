#![allow(unused)]

fn main() {

    let x: Option<i32> = Some(5);
    match x {
        Some(v) => println!("Value is: {}", v),
        _ => println!("No value"),

    }

    // instead we can write like:

    if let Some(v) = x {
        println!("Value is: {v}");

    } 

    let Some(v) = x else {
        println!("No value");
        return; 

    } //the code above must diverge i.e panic or return above

}