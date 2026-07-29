use std::f64::consts::PI;
use std::io;

const EPSILON: f64 = 1e-10;

fn main() {
    loop {
        println!(
            "Ne yapmak istiyorsunuz?\n\
        1 - Çap, Çevre ve Alan\n\
        2 - Yay ve Dilim\n\
        3 - Kiriş ve Çember Üzerindeki Nokta\n\
        4 - Nokta Çemberin İçinde mi?\n\
        5 - Teğet Uzunluğu\n\
        6 - Halka (Annulus) Alanı\n\
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

                if r <= 0.0 {
                    println!("Yarıçap pozitif olmalıdır.");
                    continue;
                }

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

                if r <= 0.0 {
                    println!("Yarıçap pozitif olmalıdır.");
                    continue;
                }

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

                if r <= 0.0 {
                    println!("Yarıçap pozitif olmalıdır.");
                    continue;
                }

                println!("Merkez açıyı girin: ");
                let mut a = String::new();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                let a: f64 = a.trim().parse().expect("Geçerli bir sayı girin");

                let rad = a.to_radians();

                let kiris = 2.0 * r * (rad / 2.0).sin();
                let x = r * rad.cos();
                let y = r * rad.sin();

                println!();
                println!("=== Sonuçlar ===");
                println!("Kiriş uzunluğu     : {:.4}", kiris);
                println!("x kordinatı        : {:.4}", x);
                println!("y kordinatı        : {:.4}", y);
                println!("=== Sonuçlar ===");
                println!();
            }
            "4" => {
                println!("Yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                if r <= 0.0 {
                    println!("Yarıçap pozitif olmalıdır.");
                    continue;
                }

                println!("x kordinatını girin: ");
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line");
                let x: f64 = x.trim().parse().expect("Geçerli bir sayı girin");

                println!("y kordinatını girin: ");
                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read line");
                let y: f64 = y.trim().parse().expect("Geçerli bir sayı girin");

                let uzaklik_kare = x.powi(2) + y.powi(2);
                let yaricap_kare = r.powi(2);

                let fark = (uzaklik_kare - yaricap_kare).abs();

                if uzaklik_kare < yaricap_kare {
                    println!();
                    println!("Nokta çemberin içindedir.");
                    println!();
                } else if fark < EPSILON {
                    println!();
                    println!("Nokta çember üzerindedir.");
                    println!();
                } else {
                    println!();
                    println!("Nokta çemberin dışındadır.");
                    println!();
                }
            }
            "5" => {
                println!("Yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                if r <= 0.0 {
                    println!("Yarıçap pozitif olmalıdır.");
                    continue;
                }

                println!("Çember merkezinden dış noktaya olan uzaklığı girin: ");
                let mut d = String::new();
                io::stdin().read_line(&mut d).expect("Failed to read line");
                let d: f64 = d.trim().parse().expect("Geçerli bir sayı girin");

                if d <= r {
                    println!("Bu noktadan teğet çizilemez.");
                } else {
                    let teget = (d.powi(2) - r.powi(2)).sqrt();
                    println!();
                    println!("=== Sonuçlar ===");
                    println!("Teğet uzunluğu     : {:.4}", teget);
                    println!("=== Sonuçlar ===");
                    println!();
                }
            }
            "6" => {
                println!("Küçük yarıçapı girin: ");
                let mut r = String::new();
                io::stdin().read_line(&mut r).expect("Failed to read line");
                let r: f64 = r.trim().parse().expect("Geçerli bir sayı girin");

                println!("Büyük yarıçapı girin: ");
                let mut buyuk_r = String::new();
                io::stdin()
                    .read_line(&mut buyuk_r)
                    .expect("Failed to read line");
                let buyuk_r: f64 = buyuk_r.trim().parse().expect("Geçerli bir sayı girin");

                if r <= 0.0 || buyuk_r <= 0.0 {
                    println!("Yarıçaplar pozitif olmalıdır.");
                    continue;
                }

                if buyuk_r <= r {
                    println!("Büyük yarıçap küçük yarıçaptan büyük olmalıdır.");
                } else {
                    let halka_alanı = PI * (buyuk_r.powi(2) - r.powi(2));
                    println!();
                    println!("=== Sonuçlar ===");
                    println!("Halka (Annulus) Alanı     : {:.4}", halka_alanı);
                    println!("=== Sonuçlar ===");
                    println!();
                }
            }
            "q" => {
                println!();
                println!("Güle güle!");
                break;
            }
            _ => {
                println!();
                println!("Lütfen 1-6 arasında bir değer veya çıkmak için q girin.");
                println!();
            }
        }
    }
}
