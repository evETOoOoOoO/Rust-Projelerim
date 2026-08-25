use std::io::{self, Write};

pub fn read_f64(mesaj: &str) -> f64 {
    loop {
        print!("{}", mesaj);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Girdi okunamadı, tekrar deneyin. ");
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(sayi) => return sayi,
            Err(_) => println!("Hatalı giriş! Lütfen geçerli bir sayı yazın."),
        }
    }
}

fn main() {
    println!("=== Pisagor Bağıntısı Hesaplayıcı ===");

    let a = read_f64("a kenarının cm olarak uzunluğunu girin: ");
    let b = read_f64("b kenarının cm olarak uzunluğunu girin: ");

    let c_kare = a.powi(2) + b.powi(2);
    let c = c_kare.sqrt();

    println!("\n--- Sonuçlar ---\n");

    println!("c'nin karesi(c²): {c_kare:.4} cm²");
    println!("Hipotenüs uzunluğu (c): {c:.4} cm");
    println!("=== Pisagor Bağıntısı Hesaplayıcı ===");
}
