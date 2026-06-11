# Rust-Projelerim

Rust ile geliştirdiğim küçük, orta ve büyük ölçekli projelerin bulunduğu monorepo. Aynı zamanda öğrenme sürecinde yaptığım denemeleri ve örnek çalışmaları da içerir.

## Amaç

- Rust dilinde pratik yapmak
- Farklı ölçeklerde proje geliştirmek
- Öğrenme sürecini tek bir repo altında toplamak
- Deneysel ve üretim seviyesindeki kodları organize etmek

## Yapı

Projeler genellikle `crates/` altında tutulur:

```bash
# Belirli bir projeyi çalıştırmak için:
cargo run -p <crate_adi>

# Tüm workspace içindeki testleri koşmak için:
cargo test
```
## Not

Bu repo sürekli gelişmektedir; bazı projeler tamamlanmamış veya deneysel olabilir.
