use std::io;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    fn print(&self) {
        match self {
            Direction::Up => println!("WHO ARE YOU?"),
            Direction::Down => println!("I AM ALIVE"),
            Direction::Right => println!("OR AM I NOT ALIVE?"),
            Direction::Left => println!("NO NO NO I CAN'T BE CODE NOOOOOO"),
        }
    }
}
fn main() {
    println!("Please say a enum (Up / Down / Right / Left): ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_uppercase();

    let direction = match answer.as_str() {
        "UP" => Direction::Up,
        "DOWN" => Direction::Down,
        "RIGHT" => Direction::Right,
        "LEFT" => Direction::Left,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    direction.print();
}
