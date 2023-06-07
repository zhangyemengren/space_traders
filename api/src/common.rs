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
pub struct Destination {
    #[serde(flatten)]
    flatten_waypoints: Waypoints,
    #[serde(rename = "systemSymbol")]
    system_symbol: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Success<T>{
    pub data: T,
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
    NoData,
}

pub fn get_token() -> String {
    // token先写死 因为web-wasm环境不支持读文件 后续在优化
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiWkhBTkdZRU1FTkdSRU4iLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wNi0wMyIsImlhdCI6MTY4NjExNjQyNiwic3ViIjoiYWdlbnQtdG9rZW4ifQ.yAK0npSzzck0-CZy2yhuMHSQwcrUTjMM36u1DYZ2LY3hv9hNHdcKPR5a0xF_n2pFR8S0EwSuYmSxTEXHYj90qhTBji5LKrnEs8lh7qmyU8hDp07B8EPTww0nqCe7QbCCzMGIzb5isdiDj6ipeoFT8MyNqbl7km62HSG-C6eHC_5v4zS6lwVIipnxc5TBHwMNc4vi7uUeM372WuKVFBL206On26P7IN8QiLp8aGXPvOFZaSlAAD8-hUixNINlpZ_-AYWTydJNBZsw0rahQUcPo69cQl86uWs3hY3ytCd7a7sqzOcXWVHG3XH7AajT2SZYgqwKKBXKBx15GdMcQI-cQQ";
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

