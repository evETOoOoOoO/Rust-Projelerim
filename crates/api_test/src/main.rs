use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .header("User-Agent", "api_test")
        .send()
        .await?;

    let status = response.status();

    let response = response.error_for_status()?;

    let todo = response.json::<Todo>().await?;

    println!("ID: {}", todo.id);
    println!("User ID: {}", todo.user_id);
    println!("Title: {}", todo.title);
    println!("Completed: {}", todo.completed);
    println!("Status: {}", status);
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
    Ok(())
}

/*
İLK SÜRÜM:

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1").await?;

    let todo = response.json::<Todo>().await?;

    println!("ID: {}", todo.id);
    println!("User ID: {}", todo.user_id);
    println!("Title: {}", todo.title);
    println!("Completed: {}", todo.completed);

    Ok(())
}
*/
