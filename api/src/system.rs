use crate::common::{ErrorRes, get};
use reqwest::Url;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Meta {
    total: u32,
    limit: u32,
    page: u32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Data{
    #[serde(flatten)]
    flatten_waypoints: Waypoints,
    #[serde(rename = "sectorSymbol")]
    sector_symbol: String,
    waypoints: Vec<Waypoints>,
    factions: Vec<Factions>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Waypoints{
    symbol: String,
    #[serde(rename = "type")]
    the_type: String,
    x: i32,
    y: i32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Factions{
    symbol: String,
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
/// 列出宇宙中所有的系统。
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
///       "sectorSymbol": "string",
///       "type": "NEUTRON_STAR", // NEUTRON_STAR RED_STAR ORANGE_STAR BLUE_STAR
///           YOUNG_STAR WHITE_DWARF BLACK_HOLE HYPERGIANT NEBULA UNSTABLE
///       "x": 0,
///       "y": 0,
///       "waypoints": [
///         {
///           "symbol": "string",
///           "type": "PLANET", // PLANETGAS GIANTMOONORBITAL STATIONJUMP GATEASTEROID FIELDNEBULADEBRIS FIELDGRAVITY WELL
///           "x": 0,
///           "y": 0
///         }
///       ],
///       "factions": [
///         {
///           "symbol": "string"
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
///       "message": "hello world",
///       "code": 0,
///       "data": {}
///     }
/// }
/// ```
pub async fn list_systems(page: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https:///api.spacetraders.io/v2/systems").expect("url parse error");
    url.query_pairs_mut().append_pair("limit", "20").append_pair("page", page);
    get(url.as_str()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_systems() {
        let res = list_systems("1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}