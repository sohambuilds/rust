#[derive(Debug)]
#[allow(dead_code)]
struct Lang {
    name:   &'static str,
    bang:   &'static str,
}

fn main() {
    let lang = "rust";
    let bang = "bruh";
    println!("Hellom {}!", lang);
    println!("Hello, {0}, {0}!", lang);


    let mang = Lang{
        name: "rust",
        bang: "bruh"

    };

    println!("{:?}", mang);
    println!("{:#?}", mang);
}