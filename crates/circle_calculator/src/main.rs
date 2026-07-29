use std::f64::consts::PI;
use std::io;

fn main() {
    loop {
        println!(
            "Ne yapmak istiyorsunuz?\n\
        1 - Çap, Çevre ve Alan\n\
        2 - Yay ve Dilim\n\
        3 - Kiriş ve Çember Üzerindeki Nokta\n\
        q - Çıkış"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input {
            "1" => {
                println!("Yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                let cap = 2.0 * r;
                let cevre = 2.0 * PI * r;
                let alan = PI * r * r;

                println!();
                println!("=== Sonuçlar ===");
                println!("Çap  : {:.4}", cap);
                println!("Çevre: {:.4}", cevre);
                println!("Alan : {:.4}", alan);
                println!("=== Sonuçlar ===");
                println!();
            }
            "2" => {
                println!("Yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                println!("Merkez açıyı girin: ");
                let mut a = String::new();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                let a: f64 = a.trim().parse().expect("Geçerli bir sayı girin");

                let cember_yay_uzunlugu = 2.0 * PI * r * a / 360.0;
                let daire_dilimi_alani = PI * r * r * a / 360.0;

                println!();
                println!("=== Sonuçlar ===");
                println!("Çember yay uzunluğu: {:.4}", cember_yay_uzunlugu);
                println!("Daire dilimi alanı : {:.4}", daire_dilimi_alani);
                println!("=== Sonuçlar ===");
                println!();
            }
            "3" => {
                println!("Yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                println!("Merkez açıyı girin: ");
                let mut a = String::new();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                let a: f64 = a.trim().parse().expect("Geçerli bir sayı girin");

                let kiris = 2.0 * r * (a.to_radians() / 2.0).sin();
                let x = r * a.to_radians().cos();
                let y = r * a.to_radians().sin();

                println!();
                println!("=== Sonuçlar ===");
                println!("Kiriş uzunluğu     : {:.4}", kiris);
                println!("x kordinatı        : {:.4}", x);
                println!("y kordinatı        : {:.4}", y);
                println!("=== Sonuçlar ===");
                println!();
            }
            "q" => {
                println!();
                println!("Güle güle!");
                break;
            }
            _ => {
                println!();
                println!("Lütfen 1, 2 veya 3 girin.");
            }
        }
    }
}
