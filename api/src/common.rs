use dotenvy::dotenv;
use std::{env};
use std::fmt::Debug;
use serde_json::Value;
use reqwest::{self, Client};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Error{
    message: String,
    code: u32,
    #[serde(default)]
    data: Value,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorRes{
    error:Error
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Meta {
    total: u32,
    limit: u32,
    page: u32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Waypoints{
    symbol: String,
    #[serde(rename = "type")]
    the_type: String,
    x: i32,
    y: i32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Success<T>{
    data: T,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SuccessVec<T>{
    data: Vec<T>,
    meta: Meta,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Response<T>{
    Success(T),
    Error(ErrorRes),
}

pub fn get_token() -> String {
    dotenv().expect(".env file not found");
    env::var("token").expect("token not found")
}

pub async fn get<T>(url: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + Debug,
{
    let c = Client::new();
    let resp = c
        .get(url)
        .header("Authorization","Bearer ".to_string() + get_token().as_str())
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(resp)
}

