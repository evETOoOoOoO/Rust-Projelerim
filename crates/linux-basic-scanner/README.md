# linux-basic-scanner — Projeyi Klonlayan İçin Kurulum

Bu proje GitHub'dan klonlandığında `quarantine/` klasörünün izinleri ve
test dosyası **otomatik gelmez** (Git izinleri garanti taşımaz, test
dosyası da `.gitignore` ile hariç tutulmuştur). Aşağıdaki adımları sırayla
uygula.

---

## 1. GNU/Linux Üzerinde Rust (cargo) Kurulumu — Dağıtıma Göre

### Arch Linux / CachyOS / Manjaro
```bash
sudo pacman -S base-devel
sudo pacman -S rust
```

### Debian / Ubuntu / Linux Mint / Pardus
```bash
sudo apt update
sudo apt install build-essential cargo
```

### Fedora
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install cargo
```

### openSUSE
```bash
sudo zypper install -t pattern devel_basis
sudo zypper install cargo
```

---

## 2. WSL Üzerinde Kurulum

Windows üzerinde WSL kullanıyorsanız projeyi doğrudan WSL terminali içerisinde çalıştırabilirsiniz.

WSL kurulu değilse PowerShell'i yönetici olarak açıp:

```powershell
wsl --install
```

komutunu çalıştırabilirsiniz.

Kurulum tamamlandıktan sonra Windows yeniden başlatma isteyebilir.

WSL terminalini açtıktan sonra kullandığınız GNU/Linux dağıtımına göre yukarıdaki Rust kurulum adımlarından uygun olanı uygulayın.

Örneğin Ubuntu veya Debian tabanlı bir WSL dağıtımında:

```bash
sudo apt update
sudo apt install build-essential cargo
```

Ardından projeyi WSL terminalinde klonlayın:

```bash
git clone https://github.com/evETOoOoOoO/Rust-Projelerim.git
cd Rust-Projelerim/crates/linux-basic-scanner
```

> **Not:** Projeyi mümkünse WSL'nin kendi GNU/Linux dosya sistemi içerisinde (`~/` gibi) çalıştırın.
>
> Örneğin:
>
> ```text
> ~/Rust-Projelerim/
> ```
>
> yerine:
>
> ```text
> /mnt/c/...
> ```
>
> altında çalıştırmaktan kaçınmak GNU/Linux dosya izinleri açısından daha tutarlı bir ortam sağlar.

---

## 3. Projeyi Klonla (TÜM GNU/Linux dağıtımlarda ortak)

```bash
git clone https://github.com/evETOoOoOoO/Rust-Projelerim.git
cd Rust-Projelerim/crates/linux-basic-scanner
```

---

## 4. İzinleri ve Test Dosyasını Kur (TÜM GNU/Linux dağıtımlarda ortak)

```bash
# 1. İzinleri tekrar ayarla (Git bunu garanti taşımaz)
chmod 750 quarantine
find quarantine -type f -exec chmod 400 {} \;

# 2. Kendi test dosyasını oluştursun
echo 'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*' > quarantine/eicar-test.com
chmod 400 quarantine/eicar-test.com

# 3. Çalıştırsın
cargo build --release
cargo run --release
```

> **Not:** `eicar-test.com` gerçek bir zararlı yazılım değildir. Antivirüs ve güvenlik yazılımlarının algılama davranışını test etmek için kullanılan standart EICAR test dosyasıdır. Sisteminizdeki antivirüs bu dosyayı oluşturulduğu anda silebilir veya karantinaya alabilir. Böyle bir durumda davranış normaldir.

---

## Beklenen Çıktı

```
✅ Temiz: .gitkeep
🚨 ZARARLI: eicar-test.com
Silinsin mi? (e/h):
```
