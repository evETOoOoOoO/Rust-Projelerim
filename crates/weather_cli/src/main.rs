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
    let client = reqwest::Client::new();
    let response1 = client
        .get("https://api.open-meteo.com/v1/forecast?latitude=39.93&longitude=32.85&current=temperature_2m&timezone=auto")
        .header("User-Agent", "weather_cli")
        .send()
        .await?;

    let response2 = client
        .get("https://geocoding-api.open-meteo.com/v1/search?name=Istanbul&count=1")
        .header("User-Agent", "api_test")
        .send()
        .await?;

    let status1 = response1.status();

    let response1 = response1.error_for_status()?;

    let weather1 = response1.json::<Weather>().await?;

    let status2 = response2.status();

    let response2 = response2.error_for_status()?;

    let geo = response2.json::<GeoResponse>().await?;

    let location = &geo.results[0];

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
    println!("Weather API : {}", status1);
    println!("Geo API     : {}", status2);
    println!("=== Weather CLI ===");

    Ok(())
}
/*

*/
