#![allow(unused)]


fn main(){
    let mut i = 0;
    loop {

        println!("This is an infinite loop. Press Ctrl+C to exit.");
        if i == 5{
            println!("Break the Loop at 5");
            break;
        }
        i+= 1;
    }

    while i<=10 {
        println!("This is a while loop. i: {}", i);
        i += 1;
    }

    for i in 0..=10 {
        println!("This is a for loop. i: {}", i);
    }


    let arr = [1, 2, 3, 4, 5];
    for a in arr{
        println!("This is a for loop with an array. a: {}", a);
    }

    let n: usize = arr.len();
    for i in 0..n {
        println!("This is a for loop with an index. arr[{}]: {}", i, arr[i]);
    }

    let v = vec![1, 2, 3, 4, 5];
    for x in &v{
        println!("This is a for loop with a vector. x: {}", x);
    }

    //   for x in v{
    //     println!("This is a for loop with a vector. x: {}", x);
    // }// this gives ownership of the vector, so we cannot use v again

    for x in v.iter(){
        println!("This is a for loop with a vector. x: {}", x);
    } // this borrows the vector, so we can use v again

    let mut i = 0;

    let z = loop {

        println!("This is an infinite loop. Press Ctrl+C to exit.");
        if i == 5{
            println!("Break the Loop at 5");
            break 99;
        }
        i+= 1;
    };

    println!("The value of z is: {z}");


    //labels
    'outer: for i in 0..5{
        'inner: for j in 0..5{
            println!("i: {}, j: {}", i, j);
            if i==1 && j==2{
                break 'outer;
            }
        }
    }
 

}