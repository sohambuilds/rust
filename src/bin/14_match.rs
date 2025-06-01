#![allow(unused)]
 

enum Animal {
    Dog,
    Cat,
    Fish,
}



fn main(){

    let x =1; 
    match x{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    match x {
        1 | 2 | 3 => println!("one, two or three"),
        _ => println!("something else"),

     } 

     match x {
        1..=3 => println!("one, two or three"),
        _ => println!("something else"),
}

    match x {
        i @ 1..=3 => println!("one, two or three with value: {}", i),
        _ => println!("something else"),
  }


    let animal = Animal::Dog;
    let animal_sound = match animal {
        Animal::Dog => println!("woof"),
        Animal::Cat => println!("meow"),
        Animal::Fish => println!("blub"),

        _ => println!("unknown animal"),
    };

    println!("Animal sound: {animal_sound:?}");// i have no idea why theres an error here if i remove the :?

    let x: Option<i32> = Some(5);
    match x{
        Some(value) => println!("Value is: {}", value),
        None => println!("No value"),
    }

    let res: Result<u32, String> = Ok(10);
    match res {
        Ok(val) => println!("Success with value: {}", val),
        Err(err) => println!("Error: {}", err),
    }


   





}