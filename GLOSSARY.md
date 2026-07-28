
# === TERİMLER SÖZLÜĞÜ ===

* **&str** = sabit metin dilimi (string slice), bellekteki metne sabit bir referanstır
* **!** = macro (kısaltılmış kod denebilir)
* **.gitignore** = git sürüm kontrol sisteminde hangi dosya veya klasörlerin takip edilmeyeceğini söyleyen ayar dosyası
* **;** = satır sonu (örn: `println!("Hello, world!");`) (her şeyde kullanılmaz)
* **Block Scope** = blok kapsamı
* **bool** = sadece `true` veya `false` değeri alabilen mantıksal veri tipi
* **Borrow Checker** = derleyicinin belleği güvende tutmak için ödünç alma kurallarını denetleyen mekanizması
* **Borrowing** = ödünç alma
* **Cargo** = rust'ın yerleşik paket yöneticisi
* **Cargo.lock** = projedeki bağımlılıkları takip eden dosya
* **Cargo.toml** = rust projesinin kalbidir; projenin kimlik kartı, ayar paneli ve alışveriş listesidir
* **cd** = Change Directory (Dizini Değiştir)
* **Channel** = kanal
* **char** = 4 baytlık yer kaplayan Unicode karakter tipi (örn: 'a', '🎨')
* **Closure** = çevresindeki değişkenleri yakalayabilen isimsiz (anonim) fonksiyonlar
* **Compile Time** = derleme zamanı
* **Compiler** = derleyici
* **Compound** = birleşik
* **const** = sabit
* **Crate** = rust'ın en küçük derleme birimi; bir kütüphane veya çalıştırılabilir program paketi
* **Debug Mode** = hata ayıklama safhası
* **Deconstruction** = ögelere ayırma
* **Dynamic Dispatch** = dinamik bağlam
* **Enum** = bir değişkenin alabileceği farklı varyasyonları tanımlayan çoklu durum veri tipi
* **Executable** = çalıştırılabilir dosya
* **Expression** = ifade
* **Function Scope** = fonksiyon kapsamı
* **Garbage Collector (GC)** = rust'ta bulunmayan çöp toplayıcı mekanizma (rust bunun yerine sahiplik modelini kullanır)
* **i32 / f64** = rust'ın varsayılan işaretli tam sayı (i32) ve ondalıklı sayı (f64) tipleri
* **Immutable** = değişemez
* **Indentation** = girinti(rust'da varsayılan 4 boşluk)
* **Intager Overflow** = tamsayı taşması
* **Key** = anahtar
* **Lifetime** = bir referansın bellekte ne kadar süre geçerli kalacağını belirten yaşam süresi
* **Loop Scope** = döngü kapsamı
* **main.exe** = derlenmiş ve çalıştırılabilir program (Linux'ta geçersiz)
* **main.pdb** = hata ayıklama dosyası
* **main.rs** = kaynak kodu
* **Match** = rust'ın güçlü ve kapsamlı örüntü eşleştirme (pattern matching) kontrol akışı ifadesi
* **Module (mod)** = kodları organize etmek ve erişimi (pub/priv) yönetmek için kullanılan yapı
* **Mutable** = değişebilir
* **Operator Overloading** = operatör aşırı yükleme
* **Option** = bir değerin var olup olmadığını (Some veya None) güvenli şekilde temsil eden yerleşik tip
* **Ownership** = sahiplik (rust'ın bellek yönetimini sağlayan ana kural seti)
* **Panic** = programın kurtarılamaz bir hata ile karşılaşarak çalışmayı durması
* **Pattern** = örüntü
* **Pattern Matching** = örüntü eşleştirme
* **Pointer** = işaretçi
* **Polymorphism** = çok biçimlilik
* **Prelude** = rust'ın her programa otomatik olarak dahil ettiği en temel standart kütüphane özellikleri
* **Release** = yayınlama
* **Remainder** = kalan
* **Result** = bir işlemin başarıyla tamamlandığını (Ok) veya hata verdiğini (Err) dönen yerleşik tip
* **Runtime**
* **Scalar** = skaler
* **Scope** = kapsam
* **Shadowing** = gölgeleme
* **Slice** = bir koleksiyonun (dizi veya string gibi) belirli bir bölümüne başvuran, boyutu dinamik referans
* **Smart Pointer** = akıllı işaretçi
* **Statement** = işlem
* **Static Dispatch** = statik bağlam
* **String** = heap bellekte büyütülebilen ve değiştirilebilen metin tipi
* **Struct** = kendi özel veri tiplerini oluşturmanı sağlayan yapılar (sınıflara benzer)
* **Signed** = işaretli
* **Thread** = iş parçacığı
* **TOML** = "Tom's Obvious, Minimal Language" ifadesinin baş harflerinden oluşur. insanların rahat okuyup yazabilmesi için tasarlanmış ayar dosyası formatıdır (.toml)
* **Trait** = nitelik
* **Tuple** = farklı veri tiplerini tek çatıda toplayan sabit uzunluktaki yapılar (örn: (5, true, 3.14))
* **Unsafe Rust** = derleyicinin bellek güvenliği kontrollerini devre dışı bıraktığın özel kod blokları (unsafe)
* **Unsigned** = işaretsiz
* **Value** = değer
* **Vector (Vec)** = dinamik olarak büyüyüp küçülebilen, heap bellekte tutulan dizi tipi
* **[dependencies]** = bağımlılık

# === TERİMLER SÖZLÜĞÜ ===

# === RUST & CARGO KOMUTLARI KILAVUZU ===

| Kategori                 | Komut                         | Açıklama / Etki                                                                                |
| :----------------------- | :---------------------------- | :--------------------------------------------------------------------------------------------- |
| **Rustup (Yönetim)**     | `rustup update`               | Rust'ı en son sürüme günceller                                                                 |
| **Rustup (Yönetim)**     | `rustup check`                | Yeni bir güncelleme var mı diye kontrol eder                                                   |
| **Rustup (Yönetim)**     | `rustup self update`          | Rustup aracının kendisini günceller                                                            |
| **Rustup (Yönetim)**     | `rustup toolchain list`       | Sistemde yüklü olan tüm Rust sürümlerini listeler                                              |
| **Rustup (Yönetim)**     | `rustup default stable`       | Varsayılan derleyici sürümünü "stable" (kararlı) yapar                                         |
| **Rustup (Yönetim)**     | `rustup override set nightly` | Bulunulan klasöre özel olarak deneysel "nightly" sürümünü tanımlar                             |
| **Rustup (Yönetim)**     | `rustup show`                 | Aktif aktif sistem durumunu ve araç zincirini gösterir                                         |
| **Rustup (Yönetim)**     | `rustup component add <bils>` | Derleyiciye clippy veya rustfmt gibi yeni araçlar/bileşenler ekler                             |
| **Derleyici (rustc)**    | `rustc main.rs`               | Cargo olmadan, tek bir kaynak kod dosyasını doğrudan derler                                    |
| **Derleyici (rustc)**    | `rustc --version`             | Yüklü olan rustc derleyicisinin sürümünü gösterir                                              |
| **Derleyici (rustc)**    | `rustc --explain <HataKodu>`  | Belirtilen derleyici hatasının detaylı nedenini ve çözümünü açıklar                            |
| **Proje Oluşturma**      | `cargo new proje_adi`         | Çalıştırılabilir (binary) yeni bir Rust projesi oluşturur                                      |
| **Proje Oluşturma**      | `cargo new proje_adi --lib`   | Yeni bir kütüphane (library) projesi oluşturur                                                 |
| **Proje Oluşturma**      | `cargo init`                  | Hali hazırda bulunulan klasörü bir Rust projesine dönüştürür                                   |
| **Derleme (Build)**      | `cargo build`                 | Projeyi hızlıca derler (Target/debug klasörüne çıktı verir)                                    |
| **Derleme (Build)**      | `cargo build --release`       | Kodu optimize eder, en hızlı çalışacak şekilde canlıya hazır derler                            |
| **Derleme (Build)**      | `cargo check`                 | Çalıştırılabilir dosya üretmeden kodu hızlıca tarar, hata var mı bakar                         |
| **Derleme (Build)**      | `cargo clean`                 | Derleme çıktılarının tutulduğu `target` klasörünü tamamen siler                                |
| **Çalıştırma**           | `cargo run`                   | Kodu önce derler, hata yoksa doğrudan çalıştırır                                               |
| **Çalıştırma**           | `cargo run --release`         | Kodu canlı sürüm modunda (hızlı ve optimize) derler ve çalıştırır                              |
| **Test Etme**            | `cargo test`                  | Projedeki tüm test fonksiyonlarını (`#[test]`) çalıştırır                                      |
| **Test Etme**            | `cargo test test_adi`         | Sadece ismi belirtilen özel bir testi çalıştırır                                               |
| **Test Etme**            | `cargo test -- --nocapture`   | Testler çalışırken kod içindeki `println!` çıktılarını ekrana basar                            |
| **Test Etme**            | `cargo test -- --ignored`     | Sadece `#[ignore]` olarak işaretlenmiş, es geçilen testleri çalıştırır                         |
| **Dokümantasyon**        | `cargo doc`                   | Projeniz ve bağımlılıklarınız için yerel HTML dokümantasyonu üretir                            |
| **Dokümantasyon**        | `cargo doc --open`            | Üretilen yerel dokümantasyonu doğrudan varsayılan tarayıcıda açar                              |
| **Dokümantasyon**        | `cargo doc --no-deps`         | Bağımlılıkları es geçerek sadece kendi yazdığınız kodun dokümanını açar                        |
| **Dokümantasyon**        | `rustup doc`                  | Tüm temel Rust kütüphane dokümantasyonunu bilgisayara indirir                                  |
| **Dokümantasyon**        | `rustup doc --book`           | Çevrimdışı resmi "Rust Programlama Dili Kitabı"nı tarayıcıda açar                              |
| **Dokümantasyon**        | `rustup doc --std`            | Çevrimdışı Standart Kütüphane (std) referans dokümanını açar                                   |
| **Bağımlılık (Cargo)**   | `cargo add <crate>`           | `Cargo.toml` dosyasına otomatik olarak yeni bir kütüphane ekler                                |
| **Bağımlılık (Cargo)**   | `cargo remove <crate>`        | Belirtilen kütüphaneyi projeden ve `Cargo.toml` içinden siler                                  |
| **Bağımlılık (Cargo)**   | `cargo update`                | Projedeki bağımlılıkları (`Cargo.lock` üzerinden) üst sürümlerine günceller                    |
| **Bağımlılık (Cargo)**   | `cargo tree`                  | Projenin kullandığı tüm kütüphaneleri ağaç yapısı şeklinde listeler                            |
| **Paket Yayınlama**      | `cargo login`                 | crates.io üzerinde paket yayınlamak için API token girişi yapar                                |
| **Paket Yayınlama**      | `cargo publish`               | Yazdığınız kütüphaneyi herkesin kullanması için crates.io'ya yükler                            |
| **Paket Yayınlama**      | `cargo publish --dry-run`     | Paketi gerçekten yayınlamadan önce bir hata var mı diye simüle eder                            |
| **Paket Yayınlama**      | `cargo yank <crate> --ver`    | Hatalı veya sorunlu bir paket sürümünü indirmelerden geri çeker                                |
| **Bilgi Alma**           | `cargo --version`             | Cargo paket yöneticisinin mevcut sürümünü gösterir                                             |
| **Bilgi Alma**           | `cargo --list`                | Cargo ile kullanabileceğiniz tüm alt komutları listeler                                        |
| **Bilgi Alma**           | `cargo help`                  | Cargo genel yardım menüsünü ekrana getirir                                                     |
| **Global Araçlar**       | `cargo install <crate>`       | Bilgisayarınıza küresel (global) bir CLI aracı/programı yükler                                 |
| **Global Araçlar**       | `cargo uninstall <crate>`     | `cargo install` ile yüklenen global bir aracı sistemden kaldırır                               |
| **Global Araçlar**       | `cargo install-update -a`     | Küresel olarak yüklenmiş tüm araçları tek komutla günceller                                    |
| **İleri Seviye**         | `cargo metadata`              | Makinelerin okuyabileceği formatta proje bağımlılık JSON çıktısı verir                         |
| **İleri Seviye**         | `cargo pkgid`                 | Projenizin yerel ve benzersiz paket kimliğini (ID) gösterir                                    |
| **İleri Seviye**         | `cargo locate-project`        | Ana `Cargo.toml` dosyasının bilgisayardaki tam dosya yolunu verir                              |
| **Kod Kalitesi & Format**| `cargo clippy`                | Kodu analiz eder; daha temiz ve performanslı yazman için akıl verir                            |
| **Kod Kalitesi & Format**| `cargo clippy --fix`          | Clippy'nin bulduğu ve düzeltebildiği uyarıları otomatik koda uygular                           |
| **Kod Kalitesi & Format**| `cargo fmt`                   | Tüm projedeki kodları resmi Rust standart stil rehberine göre hizalar                          |
| **Kod Kalitesi & Format**| `cargo fmt -- --check`        | Kod formatını kontrol eder ancak dosyalar üzerinde bir değişiklik yapmaz                       |
| **Gelişmiş Eklentiler**  | `cargo tarpaulin`             | Yazdığın testlerin kodun yüzde kaçını kapsadığunu satır satır raporlar                         |
| **Gelişmiş Eklentiler**  | `cargo watch -x run`          | Dosyaları her kaydettiğinde projeyi otomatik yeniden derler ve çalıştırır                      |


# Rust Operatörler Tablosu
# -------------------------------------------------------------------------------------------------------------------------------------------------

| [Operatör] | [Türü]         | [Açıklama]                    | [Örnek]        | [Sonuç / Etki]                                             |
| :--------- | :------------- | :----------------------------- | :------------- | :--------------------------------------------------------- |
| ==         | Karşılaştırma  | Eşittir                        | 5 == 5         | true                                                       |
| !=         | Karşılaştırma  | Eşit Değildir                  | 5 != 10        | true                                                       |
| <          | Karşılaştırma  | Küçüktür                       | 3 < 7          | true                                                       |
| >          | Karşılaştırma  | Büyüktür                       | 20 > 10        | true                                                       |
| <=         | Karşılaştırma  | Küçük Eşittir                  | 5 <= 5         | true                                                       |
| >=         | Karşılaştırma  | Büyük Eşittir                  | 8 >= 3         | true                                                       |
| &&         | Mantıksal      | VE (AND)                       | true && false  | false                                                      |
| \|\|       | Mantıksal      | VEYA (OR)                      | true \|\| false| true                                                       |
| !          | Mantıksal      | DEĞİL (NOT)                    | !true          | false                                                      |
| +=         | Bileşik Atama  | Toplayarak Ata                 | x += 5         | x = x + 5                                                  |
| -=         | Bileşik Atama  | Çıkararak Ata                  | x -= 3         | x = x - 3                                                  |
| *=         | Bileşik Atama  | Çarparak Ata                   | x *= 2         | x = x * 2                                                  |
| /=         | Bileşik Atama  | Bölerek Ata                    | x /= 4         | x = x / 4                                                  |
| %=         | Bileşik Atama  | Mod Alarak Ata                 | x %= 2         | x = x % 2                                                  |
| &          | Bellek         | Ödünç Alma (Referans)          | &x             | x'in bellek adresini verir (Read-only)                     |
| &mut       | Bellek         | Değiştirilebilir Ödünç Alma    | &mut x         | x'in değerini değiştirme izni verir                        |
| *          | Bellek         | Değer Çözme (Dereference)      | *y             | y adresindeki gerçek değere erişir                         |
| ?          | Hata Yönetimi  | Hata Yayma / Erken Dönüş       | islem()?       | Ok ise değeri çıkarır, Err ise return eder                 |
| ..         | Aralık Kontrol | Açık Aralık (Son dahil değil)  | 1..5           | 1, 2, 3, 4 sayılarını kapsar                               |
| ..=        | Aralık Kontrol | Kapalı Aralık (Son dahil)      | 1..=5          | 1, 2, 3, 4, 5 sayılarını kapsar                            |
| as         | Veri Tipi      | Tip Dönüşümü                   | x as f64       | Tam sayı olan x'i ondalıklı sayıya çevirir                 |

# -------------------------------------------------------------------------------------------------------------------------------------------------
# Rust Operatörler Tablosu
