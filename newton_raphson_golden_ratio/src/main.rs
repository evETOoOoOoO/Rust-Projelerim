fn main() {
    // Başlangıç tahmini (x)
    let mut x: f64 = 1.0;
    let iterations = 10; // Daha fazla iterasyon daha yüksek hassasiyet sağlar

    /*
    Newton-Raphson: x_{n+1} = x_n - f(x_n) / f'(x_n)
    f(x) = x^2 - x - 1
    f'(x) = 2x - 1
    */

    for _ in 0..iterations {
        // Paydayı (türevi) bir değişkene alıyoruz ki kontrol edebilelim
        let turev = 2.0 * x - 1.0;

        // Eğer türev sıfırsa, sıfıra bölme hatası almamak için programı durdur
        if turev == 0.0 {
            panic!("Hata: Türev sıfır oldu, sıfıra bölme yapılamaz!")
        }

        // Her şey yolundaysa normal hesaplamaya devam et
        x = x - (x * x - x - 1.0) / turev
    }
    println!("Newton-Raphson ile hesaplanan altın oran: {:.16}", x);
}
