#![allow(dead_code)]
#![allow(non_snake_case)]
#[cfg(target_arch = "x86_64")]
use serde::Deserialize;
use std::arch::x86_64::_rdrand64_step;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Database {
    sorular: Vec<Soru>,
}

#[derive(Debug, Deserialize)]
struct Soru {
    id: String,
    meta: Meta,
    konu: Konu,
    kazanim: Kazanim,
    zorluk: String,
    soru: SoruBilgisi,

    secenekler: HashMap<String, String>,

    cevap: Cevap,

    #[serde(rename = "egitim_formati")]
    egitim_formati: EgitimFormati,
}

#[derive(Debug, Deserialize)]
struct Meta {
    kaynak: String,
    kaynak_tipi: String,
    sayfa: Option<u32>,
    soru_no: u32,
    dogrulama: String,
    ekleyen: String,
    tarih: String,
    dosya: String,
    klasor: String,
}

#[derive(Debug, Deserialize)]
struct Konu {
    ana_kategori: String,
    alt_kategori: String,
    detay: Option<String>,
    ai_guven: f64,
}

#[derive(Debug, Deserialize)]
struct Kazanim {
    kod: String,
    aciklama: String,
    ai_guven: f64,
}

#[derive(Debug, Deserialize)]
struct SoruBilgisi {
    metin: String,
    paragraf: Option<String>,
    gorsel_gerekli: bool,
}

#[derive(Debug, Deserialize)]
struct Cevap {
    dogru: Option<String>,
    aciklama: String,
}

#[derive(Debug, Deserialize)]
struct EgitimFormati {
    instruction: String,
    input: String,
    output: String,
}

fn main() {
    if !is_x86_feature_detected!("rdrand") {
        println!("RDRAND is not supported.");
        return;
    }

    loop {
        let json = fs::read_to_string("tum_sorular.json").unwrap();

        let db: Database = serde_json::from_str(&json).unwrap();

        let mut hardware_count: u64 = 0;

        unsafe { while _rdrand64_step(&mut hardware_count) == 0 {} }

        let real_number = (hardware_count % 2441) + 1;

        let random_index = (hardware_count % db.sorular.len() as u64) as usize;

        let soru = &db.sorular[random_index];

        println!("ID           : {}", soru.id);
        println!();
        println!("Zorluk       : {}", soru.zorluk);
        println!();
        println!("Ana Kategori : {}", soru.konu.ana_kategori);
        println!("Alt Kategori : {}", soru.konu.alt_kategori);
        println!("Soru         : {}", soru.soru.metin);

        if let Some(paragraf) = &soru.soru.paragraf {
            println!("\nParagraf:\n{}\n", paragraf);

            for harf in ["A", "B", "C", "D"] {
                if let Some(secenek) = soru.secenekler.get(harf) {
                    println!("{} ) {}", harf, secenek);
                }
            }
        } else {
            println!("\n(Bu soruda paragraf yok)\n");
        }

        println!("\n1. Yeni soru  2. Çıkış");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_uppercase();

        match input.as_str() {
            "1" => {
                println!();
                continue;
            }
            "2" => {
                break;
            }
            _ => {
                println!("Invalid option chosen. ");
                break;
            }
        }
    }
}
