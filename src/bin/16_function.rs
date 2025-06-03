#![allow(unused)]

fn add(x:i32, y:i32) -> i32 {
    x + y
}

fn print(){
    println!("no output")
}

fn forever() -> ! {
    loop {
        println!("This will run forever");
    }}

fn crash() -> ! {
    panic!("This function will cause a crash");
}
fn main() {
    let a = 5;
    let b = 10;
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    print();

    //diverge 
    forever();
    crash();

    

}