use std::io;

fn main() {
    println!("Please say your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
    println!("Hello, {name}!");

    println!("Do you wanna talk to me? (Y/n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_uppercase();

    println!("Ok");

    match input.as_str() {
        "Y" => {
            println!("We can talk ");
            println!("How are you?");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            println!("I understand, you said: {}", input.trim());
        }
        "N" => {
            println!("WE CANT TALK! ");
        }
        _ => {
            panic!("Invalid option chosen. ");
        }
    }
}
