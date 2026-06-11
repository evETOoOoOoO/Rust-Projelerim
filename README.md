# Rust-Projelerim

![Rust](https://shields.io)
![License](https://shields.io)
![CI](https://shields.io)
![Postman](https://shields.io)

Rust ile geliştirdiğim küçük, orta ve büyük ölçekli projelerin bulunduğu monorepo. Aynı zamanda öğrenme sürecinde yaptığım denemeleri ve örnek çalışmaları da içerir.

## Amaç

- Rust dilinde pratik yapmak
- Farklı ölçeklerde proje geliştirmek
- Öğrenme sürecini tek bir repo altında toplamak
- Deneysel ve üretim seviyesindeki kodları organize etmek

## Yapı

Bu depo bir **Cargo Workspace** mimarisi kullanmaktadır. Projeler genellikle `crates/` dizini altında tutulur. 

Ana dizindeyken projeleri yönetmek için şu komutları kullanabilirsiniz:

```bash
# Belirli bir projeyi çalıştırmak için:
cargo run -p <crate_adi>

# Tüm workspace içindeki testleri koşmak için:
cargo test
```

## Not

Bu repo sürekli gelişmektedir; bazı projeler tamamlanmamış veya deneysel olabilir.
