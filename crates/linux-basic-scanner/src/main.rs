use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    let zararlı_hashler = ["131f95c51cc819465fa1797f6ccacf9d494aaaff46fa3eac73ae63ffbdfd8267"];

    for entry in fs::read_dir("quarantine")? {
        let path = entry?.path();

        if !path.is_file() {
            continue;
        }

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 256 * 1024];

        loop {
            let bytes_okundu = reader.read(&mut buffer)?;
            if bytes_okundu == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_okundu]);
        }

        let hash = hex::encode(hasher.finalize());
        let dosya_adi = path.file_name().unwrap().to_string_lossy();

        if zararlı_hashler.contains(&hash.as_str()) {
            println!("🚨 ZARARLI: {}", dosya_adi);

            print!("Silinsin mi? (e/h): ");
            io::stdout().flush().ok();
            let mut cevap = String::new();
            io::stdin().read_line(&mut cevap).ok();

            match cevap.trim() {
                "e" => {
                    fs::remove_file(&path)?;
                    println!("Silindi.");
                }
                _ => {
                    println!("Silinmedi.");
                }
            }
        } else {
            println!("✅ Temiz: {}", dosya_adi);
        }
    }

    Ok(())
}
