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

    let json = fs::read_to_string("tum_sorular.json").unwrap();

    let db: Database = serde_json::from_str(&json).unwrap();

    let mut hardware_count: u64 = 0;

    unsafe { while _rdrand64_step(&mut hardware_count) == 0 {} }

    let real_number = (hardware_count % 2441) + 1;

    let random_index = (hardware_count % db.sorular.len() as u64) as usize;

    let soru = &db.sorular[random_index];

    println!("ID: {}", soru.id);
    println!("Soru: {}", soru.soru.metin);

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
}
