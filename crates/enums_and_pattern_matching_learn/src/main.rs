use std::io;

enum Direction {
    Up,
    Down,
    Right,
    Left,
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

    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Right => println!("Moving right"),
        Direction::Left => println!("Moving left"),
    }
}
