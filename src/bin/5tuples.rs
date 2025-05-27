#![allow(unused_variables)]

fn main(){

    let t1: (i32,f64,char) = (42,3.14,'a');

    //let (a,b,c) = t;
    let ( _,b,_) = t1;

    let t = ();

    let nested = ((1,2),(3,4));
    let nested2 = ((1,2), (3.14, 'a', true));

    let (x,y) = nested;
    println!("x: {}, y: {}", t1.0,t1.1);
    println!("nested.0.0: {}, nested.0.1: {}", nested.0 .0, nested.0 .1);


}