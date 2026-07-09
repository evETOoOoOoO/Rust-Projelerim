use std::io;

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Lütfen geçerli bir sayı gir."),
        }
    }
}

fn main() {
    loop {
        println!("\n=== Quadratic Solver ===");
        println!("1. Denklem çöz ");
        println!("2. Çıkış ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                let a = read_number("a değerini gir:");

                const EPSILON: f64 = 1e-10;

                if a.abs() < EPSILON {
                    println!("Bu ikinci dereceden denklem değildir.");
                    return;
                }

                let b = read_number("b değerini gir:");
                let c = read_number("c değerini gir:");

                let delta = b * b - 4.0 * a * c;

                println!("Delta = {}", delta);

                if delta > EPSILON {
                    println!("İki kök var.");

                    let sqrt_delta = delta.sqrt();
                    let denominator = 2.0 * a;

                    let x1 = (-b + sqrt_delta) / denominator;
                    let x2 = (-b - sqrt_delta) / denominator;

                    println!("x1 = {}", x1);
                    println!("x2 = {}", x2);
                } else if delta.abs() < EPSILON {
                    println!("Tek kök var.");

                    let x = -b / (2.0 * a);

                    println!("x = {}", x);
                } else {
                    println!("Gerçek kök yok.");
                }
            }
            "2" => {
                println!("Görüşürüz!");
                break;
            }
            _ => {
                println!("Geçersiz seçim.");
            }
        }
    }
}
