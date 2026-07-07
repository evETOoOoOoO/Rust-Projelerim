use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
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
    let client1 = reqwest::Client::new();

    let response1 = client1
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .header("User-Agent", "api_test")
        .send()
        .await?;

    let status1 = response1.status();

    let response1 = response1.error_for_status()?;

    let todo1 = response1.json::<Todo>().await?;

    println!("----------------");
    println!("ID: {}", todo1.id);
    println!("User ID: {}", todo1.user_id);
    println!("Title: {}", todo1.title);
    println!("Completed: {}", todo1.completed);
    println!("Status: {}", status1);
    //--------------------------------------
    let client2 = reqwest::Client::new();

    let response2 = client2
        .get("https://jsonplaceholder.typicode.com/users")
        .header("User-Agent", "api_test")
        .send()
        .await?;

    let status2 = response2.status();

    let response2 = response2.error_for_status()?;

    let users = response2.json::<Vec<User>>().await?;

    println!("----------------");
    println!("All user status: {}", status2);
    println!("----------------");

    for user in users {
        println!("ID: {}", user.id);
        println!("Name: {}", user.name);
        println!("Username: {}", user.username);
        println!("Email: {}", user.email);
        println!("----------------");
    }
    //--------------------------------------
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
