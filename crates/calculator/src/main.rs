use std::io;

fn main() {
    println!("Please say your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {name}!");

    let name = name.trim();

    println!("Are you wan't to do a math operation? (Y/N): ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_uppercase();

    match answer.as_str() {
        "Y" => {
            println!("Math mode");
            println!(
                "Which mathematical operation would you like to perform (addition, subtraction, multiplication, division, square root ( A / S / M / D / SR )?"
            );
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice = choice.trim().to_uppercase();
            match choice.as_str() {
                "A" => {
                    println!("Addition mode");
                    //Toplama işlemi
                }
                "S" => {
                    println!("Subtraction mode");
                    //Çıkarma işlemi
                }
                "M" => {
                    println!("Multiplication mode");
                    //Çarpma işlemi
                }
                "D" => {
                    println!("Division mode");
                    //Bölme işlemi
                }
                "SR" => {
                    println!("Square root mode");
                    //Karekök alma işlemi
                }
                _ => {
                    println!("Invalid choice");
                }
            }
        }
        "N" => {
            println!("Goodbye, {name}!");
        }
        _ => {
            println!("Invalid choice");
        }
    }
}
/*
fn main() {
    // Bu fonksiyon değişkenler ve veri tipleri ile ilgili bir proje örneği sunar.
    println!("Please say your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {name}!");

    let name = name.trim();

    println!("Are you wan't to do a math operation? (Y/N): ");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_uppercase();

    if answer == "Y" {
        println!(
            "Which mathematical operation would you like to perform (addition, subtraction, multiplication, division, square root ( A / S / M / D / SR )?"
        );

        let multiplier = 10000.0;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().to_uppercase();

        if choice == "A" {
            println!("Please enter the first number: ");
            let mut num1_str = String::new();
            io::stdin()
                .read_line(&mut num1_str)
                .expect("Failed to read line");

            let num1_f64: f64 = num1_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num1_scaled = (num1_f64 * multiplier).round() as i64;

            println!("Please enter the second number: ");
            let mut num2_str = String::new();
            io::stdin()
                .read_line(&mut num2_str)
                .expect("Failed to read line");

            let num2_f64: f64 = num2_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num2_scaled = (num2_f64 * multiplier).round() as i64;
            let whole_number_scaled = num1_scaled + num2_scaled;

            let final_result = whole_number_scaled as f64 / multiplier;
            println!("The whole number is: {}", final_result);
        } else if choice == "S" {
            println!("Please enter the first number: ");
            let mut num1_str = String::new();
            io::stdin()
                .read_line(&mut num1_str)
                .expect("Failed to read line");

            let num1_f64: f64 = num1_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num1_scaled = (num1_f64 * multiplier).round() as i64;

            println!("Please enter the second number: ");
            let mut num2_str = String::new();
            io::stdin()
                .read_line(&mut num2_str)
                .expect("Failed to read line");

            let num2_f64: f64 = num2_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num2_scaled = (num2_f64 * multiplier).round() as i64;
            let whole_number_scaled = num1_scaled - num2_scaled;

            let final_result = whole_number_scaled as f64 / multiplier;
            println!("The whole exact number is: {}", final_result);
        } else if choice == "D" {
            println!("Please enter the first number (Dividend): ");
            let mut num1_str = String::new();
            io::stdin()
                .read_line(&mut num1_str)
                .expect("Failed to read line");

            let num1_f64: f64 = num1_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num1_scaled = (num1_f64 * multiplier).round() as i64;

            println!("Please enter the second number (Divisor): ");
            let mut num2_str = String::new();
            io::stdin()
                .read_line(&mut num2_str)
                .expect("Failed to read line");

            let num2_f64: f64 = num2_str
                .trim()
                .parse()
                .expect("Please enter a valid number");

            let num2_scaled = (num2_f64 * multiplier).round() as i64;

            if num2_scaled == 0 {
                println!("Error: Division by zero is not allowed!");
                return;
            }

            let whole_number_scaled = (num1_scaled as f64 * multiplier) / num2_scaled as f64;

            let final_result = whole_number_scaled as f64 / multiplier;
            println!("The whole exact number is: {}", final_result);
        }
    } else {
        println!("Goodbye, {name}!");
    }
}
*/
