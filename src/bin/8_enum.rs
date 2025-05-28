#[derive(Debug,PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl{h: u8, s:u8, l :u8},
     

}

fn main() {


    let color: Color = Color::Rgba(255, 0, 0, 1.0);
    let color = Color::Hex(String::from("#FF0000"));
    let color = Color::Hsl {h:0,s:100,l:50};

    println!("Color: {:?}", color);

    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red != Color::Blue);
    println!("{}", Color::Red == Color::Red);

    //Option = Some(T) | None
    //Result = Ok(T) | Err(E)
    let x: Option<i32> = None;
    let x: Option<i32> = Some(42);

    println!("Option x: {:?}", x);

    let y: Result<i32, String> = Ok(42);
    let y: Result<i32, String> = Err(String::from("Error occurred"));
    println!("Result y: {:?}", y);
}