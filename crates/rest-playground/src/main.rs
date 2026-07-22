#![allow(dead_code)]
#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub maiden_name: String,
    pub age: u8,
    pub gender: String,
    pub email: String,
    pub phone: String,
    pub username: String,
    pub height: f64,
    pub weight: f64,
    pub birth_date: String,
    pub address: Address,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub city: String,
    pub state: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response1 = client
        .get("https://dummyjson.com/users")
        .header("User-Agent", "rest-playground")
        .send()
        .await?;

    let status1 = response1.status();

    let response1 = response1.error_for_status()?;

    let api_response = response1.json::<ApiResponse>().await?;

    for user in api_response.users {
        println!("{} {} ({})", user.first_name, user.last_name, user.email,)
    }

    Ok(())
}
