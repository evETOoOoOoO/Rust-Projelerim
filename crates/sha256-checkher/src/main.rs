use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("sample4.json").unwrap();

    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 64 * 1024];

    loop {
        let bytes_okundu = reader.read(&mut buffer).unwrap();

        if bytes_okundu == 0 {
            break;
        }

        println!("{} byte okundu", bytes_okundu);
    }
}
