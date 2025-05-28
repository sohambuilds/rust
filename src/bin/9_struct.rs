#![allow(unused)]

#derive(Debug, PartialEq, Eq)
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32,f32,f32);

struct Empty;

#derive(Debug, PartialEq, Eq)
struct Circle {
    center: Point,
    radius: u32,

}

fn main(){

    let p = Point{x:1.0,y: 2.0};
    println!("point.x = {}, point.y = {}",p.x, p.y);

    let p3d = Point3d(1.0,2.0,3.0);

    println!("Point3d: ({}, {}, {})", p3d.0, p3d.1, p3d.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 5,
    }

    println!("{:?}", circle);

    ///shortcut struct initialization
    let x = 1.0;
    let y = 1.0;

    let p = Point {x,y};


    //copy fields
    let po = Point {x: 2.0, y: 1.0};
    let p1 = Point {x: po.x, ..p0}; //update the field x, and fill the rest with p0's values
     


    let mut p = Point {x: 0.0, y: 0.0};
    p.x += 1.0;
    p.y = 10.0; //mutable field update

    


}