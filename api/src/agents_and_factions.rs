use crate::common::{ErrorRes, get};
use reqwest::Url;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Meta {
    total: u32,
    limit: u32,
    page: u32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Traits{
    symbol: String,
    name: String,
    description: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data{
    #[serde(flatten)]
    flatten_traits: Traits,
    headquarters: String,
    traits: Vec<Traits>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Success{
    data: Vec<Data>,
    meta: Meta,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Response{
    Success(Success),
    Error(ErrorRes),
}
/// 列出游戏中所有发现的派系。
/// # method
/// GET
/// # request
/// ### Query
/// ```json
/// {
///     limit: u32,
///     page: u32,
/// }
/// ```
/// # response
/// ### success
/// ```json
/// {
///   "data": [
///     {
///       "symbol": "string",
///       "name": "string",
///       "description": "string",
///       "headquarters": "string",
///       "traits": [
///         {
///           "symbol": "BUREAUCRATIC",
///           "name": "string",
///           "description": "string"
///         }
///       ]
///     }
///   ],
///   "meta": {
///     "total": 0,
///     "page": 0,
///     "limit": 0
///   }
/// }
/// ```
/// ### fail
/// ```json
/// {
///    "error": {
///       "message": "reasons",
///       "code": 0,
///       "data": {}
///     }
/// }
/// ```
pub async fn list_factions(page: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https://api.spacetraders.io/v2/factions").expect("url parse error");
    url.query_pairs_mut().append_pair("limit", "20").append_pair("page", page);
    get(url.as_str()).await
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_factions() {
        let res = list_factions("1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}
