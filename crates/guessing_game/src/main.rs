#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdrand64_step;
use std::io;

fn main() {
    println!("I will now guess the number you are thinking of. (1 - 100)");
    println!("Hmm, I'm thinking.");

    let mut hardware_count: u64 = 0;

    unsafe { while _rdrand64_step(&mut hardware_count) == 0 {} }

    // 1-100 ARASINA SIKIŞTIRMA FORMÜLÜ:
    let real_number = (hardware_count % 100) + 1;

    println!("Was this the number you were thinking of: {}", real_number);

    println!("Did I guess correctly? (Y / n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_uppercase();

    match input.as_str() {
        "Y" => {
            print!("OH MY GOD I CAN'T BELIEVE IT YEEEY");
        }
        "N" => {
            println!("HOW?!?!?!?!?");
            return;
        }
        _ => {
            panic!("Invalid option chosen. ");
        }
    }
}
/*
TR:

Bu proje, bilgisayarın işlemcisinde (CPU) bulunan fiziksel termal gürültü çipini kullanarak 1 ile 100 arasında tamamen gerçek ve tahmin edilemez rastgele sayılar üretir ve kullanıcının kafasında tuttuğu tahminin programın tahminiyle aynı olup olmadığını sorar.

Yazılımsal (psödo) rastgele sayı üreteçlerinin aksine, doğrudan donanım seviyesindeki (`RDRAND`) rastgelelik sinyallerini kullanır.

Gereksinimler

Bu kodun çalışabilmesi için sisteminizin aşağıdaki şartları sağlaması gerekir:
Mimari: Sadece `x86_64` (Intel veya AMD 64-bit) işlemciler.
Destek: İşlemcinizin `RDRAND` komut setini desteklemesi gerekir (Modern tüm Intel/AMD işlemciler destekler).
Not: Apple Silicon (M1/M2/M3) gibi ARM tabanlı işlemcilerde mimari uyuşmazlığı nedeniyle derlenmez.

NOT: BU PROJE @HüseyinBABAL YOUTUBE KANALINDAKİ "Rust Öğreniyoruz #1 - 10 Dakikada İlk Projeni Yaz! (10.000 Aboneye Özel)" PROJESİNDEN ESİNLENİLEREK YAPILMIŞTIR.

EN:

This project generates truly authentic and unpredictable random numbers between 1 and 100 by utilizing the physical thermal noise source found in the computer's processor (CPU).

Unlike software-based (pseudo) random number generators, it directly utilizes randomness signals at the hardware level (`RDRAND`).

Requirements

For this code to run, your system must meet the following criteria:
Architecture: `x86_64` (Intel or AMD 64-bit) processors only.
Support: Your processor must support the `RDRAND` instruction set (supported by all modern Intel/AMD processors).
Note: It will not compile on ARM-based processors, such as Apple Silicon (M1/M2/M3), due to architectural incompatibility.

NOTE: THIS PROJECT WAS INSPIRED BY THE "Rust Öğreniyoruz #1 - 10 Dakikada İlk Projeni Yaz! (10.000 Aboneye Özel)" PROJECT ON @HüseyinBABAL'S YOUTUBE CHANNEL.
*/
