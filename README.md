# Rust-Projelerim

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
# [TR] Belirli bir projeyi çalıştırmak için:
cargo run -p <crate_adi>

# [TR] Tüm workspace içindeki testleri çalıştırmak için:
cargo test
```

## Not

Bu repo sürekli gelişmektedir; bazı projeler tamamlanmamış veya deneysel olabilir.

---

# Rust-Projects

A monorepo containing small, medium, and large-scale projects developed with Rust. It also includes experimental works and sample studies created during the learning process.

## Purpose

- Practicing the Rust programming language
- Developing projects of various scales
- Consolidating the learning process under a single repository
- Organizing both experimental and production-grade code

## Structure

This repository utilizes a **Cargo Workspace** architecture. Projects are generally maintained under the `crates/` directory. 

You can use the following commands from the root directory to manage the projects:

```bash
# [EN] To run a specific project/crate:
cargo run -p <crate_name>

# [EN] To run all tests within the workspace:
cargo test
```

## Note

This repository is continuously evolving; some projects may be incomplete or experimental.

