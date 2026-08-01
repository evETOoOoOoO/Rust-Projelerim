use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Current {
    time: String,
    temperature_2m: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    timezone: String,
    elevation: f64,
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    timezone: String,
    population: u64,
    country: String,
}

#[derive(Debug, Deserialize)]
struct GeoResponse {
    results: Vec<Location>,
}
/*
    HTTP DURUM KODLARI:
    ------------------------------------
    | Kod | Anlamı                     |
    | --- | -------------------------- |
    | 200 | Başarılı                   |
    | 201 | Oluşturuldu (POST)         |
    | 400 | Hatalı istek               |
    | 401 | Yetkisiz (API key yok vb.) |
    | 403 | Yasak                      |
    | 404 | Bulunamadı                 |
    | 500 | Sunucu hatası              |
    ------------------------------------
*/
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        println!("Napmak istersiniz?( Şehir görmek(1)  / Çıkış(q) )");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "1" => {
                println!("Hangi şehri görmek istersiniz?");
                let mut sehir = String::new();
                io::stdin()
                    .read_line(&mut sehir)
                    .expect("Failed to read line");
                let sehir = sehir.trim().to_lowercase();

                let client = reqwest::Client::new();

                let geo_url = format!(
                    "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
                    sehir
                );
                let response2 = client
                    .get(&geo_url)
                    .header("User-Agent", "weather_cli")
                    .send()
                    .await?;
                let response2 = response2.error_for_status()?;
                let geo = response2.json::<GeoResponse>().await?;
                let location = &geo.results[0];

                let weather_url = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m&timezone=auto",
                    location.latitude, location.longitude
                );
                let response1 = client
                    .get(&weather_url)
                    .header("User-Agent", "weather_cli")
                    .send()
                    .await?;
                let response1 = response1.error_for_status()?;
                let weather1 = response1.json::<Weather>().await?;

                println!("=== Weather CLI ===");
                println!("Saat        : {}", weather1.current.time);
                println!("Timezone    : {}", weather1.timezone);
                println!("Rakım       : {} m", weather1.elevation);
                println!("Sıcaklık    : {}°C", weather1.current.temperature_2m);
                println!("Şehir       : {}", location.name);
                println!("Ülke        : {}", location.country);
                println!("Enlem       : {}", location.latitude);
                println!("Boylam      : {}", location.longitude);
                println!("Nüfus       : {}", location.population);
                println!("Geo Timezone: {}", location.timezone);
                println!("Geo Rakım   : {} m", location.elevation);
                println!("Weather API : {}", weather_url);
                println!("Geo API     : {}", geo_url);
                println!("=== Weather CLI ===");
            }
            "q" => {
                println!("Görüşürüz!");
                return Ok(());
            }
            _ => {
                println!("Geçersiz seçim, tekrar deneyin.");
            }
        }
    }
}
