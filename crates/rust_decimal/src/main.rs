use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Decimal {
    value: i128,
    scale: u32,
}

impl Decimal {
    fn parse(input: &str) -> Result<Decimal, String> {
        let is_negative = input.starts_with('-');
        let input = input.trim_start_matches('-'); // işareti ayır

        let parts: Vec<&str> = input.split('.').collect();

        let (int_part, frac_part) = match parts.len() {
            1 => (parts[0], ""),       // "5" gibi noktasız girdi
            2 => (parts[0], parts[1]), // "5.25" gibi normal girdi
            _ => return Err("Geçersiz format".to_string()),
        };

        let scale = frac_part.len() as u32;
        let combined = format!("{}{}", int_part, frac_part);

        let mut value: i128 = combined.parse().map_err(|_| "Sayı parse edilemedi")?;

        if is_negative {
            value = -value;
        }

        Ok(Decimal { value, scale })
    }

    fn selam_soyle(&self) {
        println!("Ben bir Decimal'im, değerim: {:?}", self);
    }

    fn to_string_custom(&self) -> String {
        let is_negative = self.value < 0;
        let abs_value = self.value.abs();
        let mut value_str = abs_value.to_string();

        while value_str.len() <= self.scale as usize {
            value_str = format!("0{}", value_str);
        }

        println!("İşaret negatif mi: {}", is_negative);
        println!("Mutlak değer: {}", abs_value);
        println!("String hali: {}", value_str);
        println!("String uzunluğu: {}", value_str.len());
        println!("Scale: {}", self.scale);

        String::new()
    }
}

fn main() {
    println!("Bir sayı girin:");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim().to_uppercase();

    let result = Decimal::parse(&answer).unwrap();
    result.selam_soyle();
    result.to_string_custom();
}
/*
fn main() {
    let test_cases = vec!["-12.34", "5", "92", "-0.32", "abc", "-0.5", "12.34.62"];

    for case in test_cases {
        let result = Decimal::parse(case);
        println!("Input: {}, Result: {:?}", case, result);
    }
}

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
