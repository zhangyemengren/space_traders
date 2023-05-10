use crate::common::{get_token, ErrorRes};
use std::{collections::HashMap};
use reqwest::{self, Client};

#[derive(Debug, serde::Deserialize)]
struct Meta {
    total: u32,
    limit: u32,
    page: u32,
}
#[derive(Debug, serde::Deserialize)]
struct Traits{
    symbol: String,
    name: String,
    description: String,
}
#[derive(Debug, serde::Deserialize)]
struct Data{
    #[serde(flatten)]
    flatten_traits: Traits,
    headquarters: String,
    traits: Vec<Traits>,
}
#[derive(Debug, serde::Deserialize)]
struct Success{
    data: Vec<Data>,
    meta: Meta,
}
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum Response{
    Success(Success),
    Error(ErrorRes),
}
///Viewing factions
///
///
///
pub async fn view_factions() -> Result<Response, Box<dyn std::error::Error>> {
    let c = Client::new();
    let resp = c.get("https://api.spacetraders.io/v2/factions")
        .header("Authorization","Bearer ".to_string() + get_token().as_str())
        .send()
        .await?
        .json::<Response>()
        .await?;
    Ok(resp)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let ip = view_factions().await.unwrap();
        println!("{:?}", ip);
        assert!(true);
    }
}
