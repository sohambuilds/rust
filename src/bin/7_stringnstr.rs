fn main(){

let message: String = String::from("Hello Rust");
let len: usize = message.len();

println!("Message: {}, Length: {}", message, len);


let s: &str = &message[0..5];
let len: usize = s.len();
println!("Substring: {}, Length: {}", s, len);

let hello: &str = "Hello";

let multiline: &str = r#"Hello
this is 
multiline

"#;

println!("Multiline String: {}", multiline);

//deref coersion
let msg: String = "Hello Rust".to_string();
let s: &str = &msg; // &String to &str conversion




let mut msg: String = "Hello Rust".to_string();
msg += " is great!";
println!("Modified Message: {}", msg);

//string interpolation

let lang = "Rust";
let emoji = "😊";

let msg = format!("Hello, {}{}", lang, emoji);
println!("{}", msg);
}


