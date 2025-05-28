#![allow(unused)]

#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}


impl Point {

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x *self.x + self.y * self.y).sqrt()
    }
     
}

fn main() {
    let mut p = Point::zero();
    println!("Point: {:?}", p);
    p.move_to(3.0, 4.0);
    println!("Moved Point: {:?}", p);

    let distance = p.dist();
    println!("Distance from origin: {}", distance);
}