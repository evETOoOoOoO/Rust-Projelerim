use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("sample4.json").unwrap();

    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];

    loop {
        let bytes_okundu = reader.read(&mut buffer).unwrap();

        if bytes_okundu == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_okundu]);
    }

    let sonuc = hasher.finalize();

    println!("{}", hex::encode(sonuc));
}
