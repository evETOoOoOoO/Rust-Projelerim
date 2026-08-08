# linux-basic-scanner — Projeyi Klonlayan İçin Kurulum

Bu proje GitHub'dan klonlandığında `quarantine/` klasörünün izinleri ve
test dosyası **otomatik gelmez** (Git izinleri garanti taşımaz, test
dosyası da `.gitignore` ile hariç tutulmuştur). Aşağıdaki adımları sırayla
uygula.

---

## 1. Rust (cargo) Kurulumu — Dağıtıma Göre

### Arch Linux / CachyOS / Manjaro
```bash
sudo pacman -S rust
```

### Debian / Ubuntu / Linux Mint
```bash
sudo apt update
sudo apt install cargo
```

### Fedora
```bash
sudo dnf install cargo
```

### openSUSE
```bash
sudo zypper install cargo
```

---

## 2. Projeyi Klonla (TÜM dağıtımlarda ortak)

```bash
git clone https://github.com/evETOoOoOoO/Rust-Projelerim.git
cd Rust-Projelerim/crates/linux-basic-scanner
```

---

## 3. İzinleri ve Test Dosyasını Kur (TÜM dağıtımlarda ortak)

```bash
cd linux-basic-scanner

# 1. İzinleri tekrar ayarla (Git bunu garanti taşımaz)
chmod 750 quarantine
find quarantine -type f -exec chmod 640 {} \;

# 2. Kendi test dosyasını oluştursun
echo 'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*' > quarantine/eicar-test.com
chmod 640 quarantine/eicar-test.com

# 3. Çalıştırsın
cargo build --release
cargo run --release
```

---

## Beklenen Çıktı

```
✅ Temiz: .gitkeep
🚨 ZARARLI: eicar-test.com
Silinsin mi? (e/h):
```
