use std::fmt::Debug;
use serde_json::Value;
use reqwest::{self, Client};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Error{
    message: String,
    code: u32,
    #[serde(default)]
    data: Value,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ErrorRes{
    error:Error
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Meta {
    total: u32,
    limit: u32,
    page: u32,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Waypoints{
    symbol: String,
    #[serde(rename = "type")]
    the_type: String,
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Destination {
    #[serde(flatten)]
    flatten_waypoints: Waypoints,
    #[serde(rename = "systemSymbol")]
    system_symbol: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Success<T>{
    pub data: T,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SuccessVec<T>{
    data: Vec<T>,
    meta: Meta,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Response<T>{
    Success(T),
    Error(ErrorRes),
    NoData,
}

pub fn get_token() -> String {
    // token先写死 因为web-wasm环境不支持读文件 后续在优化
    let token ="eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiWkhBTkdZRU1FTkdSRU4iLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wNi0xMCIsImlhdCI6MTY4NjU1NTI5Mywic3ViIjoiYWdlbnQtdG9rZW4ifQ.DZt4gbbL_gUEmh544mgSY0czMg5SGpQW1t4IWAO77c3S9oGTRMZ5H5Tbj3D8fnvwf2FlkI8k_Vq5TVCEpZULyL7AIeAj8TfwmvnbRRJbfahPkYp7Jo6OtL5qHKInm65PlBT6I0hsmI9YcjTrWXFSIN9wM0l8FzodVaGNuJppiUmfLjm9FEhfs8juOZQ3Nf36x8vzRLMzLRYUaQOZ7W96J8Bcy6e9ROURILJj0AXfQr6W2KU3NYSYf3IQi3qfBltaljJRXbcE_TDJ1338ovkwScKGr6vCqZK-hxUziLi4eCRe7CyX0TlLOh9EQ2fs50--suXt1A62ryW49gklw3pg2A";
    token.to_string()
}

pub async fn get<T>(url: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + Debug,
{
    let c = Client::new();
    let token = get_token();
    let resp = c
        .get(url);
    let resp = if token.is_empty(){
        resp
    } else {
        resp.header("Authorization","Bearer ".to_string() + token.as_str())
    };
    let resp = resp
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(resp)
}

pub async fn post<T, U>(url: &str, body: U) -> Result<T,Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + Debug,
    U: serde::Serialize,
{
    let c = Client::new();
    let token = get_token();
    let resp = c
        .post(url);
    let resp = if token.is_empty(){
        resp
    } else {
        resp.header("Authorization","Bearer ".to_string() + token.as_str())
    };
    let resp = resp
        .json(&body)
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(resp)
}

