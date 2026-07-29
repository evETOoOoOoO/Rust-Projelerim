// BU PROJE Dr. Aydın Erden'İN "RUST İLE PROGRAMLAMA" KİTABINDAN ÖĞRENİLEN BİLGİLER İLE OLUŞTURULMUŞTUR

#![allow(dead_code)]

fn main() {
    let sayi1: i32 = 5;
    let sayi2: i32 = sayi1 * 2;
    let sayi3: i32 = sayi2 + 3;

    let sonuc = basit_aritmetik(sayi1, sayi2, sayi3);

    let a: i32 = {
        let mut b: i32 = 42;
        b += 1;
        b
    };

    let sonuclu_a: i32 = sonuc + a;

    println!("{:?}", sonuclu_a);
}

fn basit_aritmetik(sayi1: i32, sayi2: i32, sayi3: i32) -> i32 {
    sayi1 + sayi2 + sayi3
}
