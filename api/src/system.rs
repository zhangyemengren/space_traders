use crate::common::{get, Waypoints, Response, SuccessVec, Success};
use reqwest::Url;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Data{
    #[serde(flatten)]
    pub flatten_waypoints: Waypoints,
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,
    pub waypoints: Vec<Waypoints>,
    pub factions: Vec<Factions>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Factions{
    pub symbol: String,
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
///           "type": "PLANET", // PLANETGAS GIANTMOONORBITAL STATIONJUMP GATEASTEROID
///               FIELDNEBULADEBRIS FIELDGRAVITY WELL
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
///       "message": "reasons",
///       "code": 0,
///       "data": {}
///     }
/// }
/// ```
pub async fn list_systems(page: String) -> Result<Response<SuccessVec<Data>>, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https://api.spacetraders.io/v2/systems").expect("url parse error");
    url.query_pairs_mut().append_pair("limit", "20").append_pair("page", &page);
    get(url.as_str()).await
}

/// 获取系统的详细信息。
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
///   "data": {
///       "symbol": "string",
///       "sectorSymbol": "string",
///       "type": "NEUTRON_STAR", // NEUTRON_STAR RED_STAR ORANGE_STAR BLUE_STAR
///           YOUNG_STAR WHITE_DWARF BLACK_HOLE HYPERGIANT NEBULA UNSTABLE
///       "x": 0,
///       "y": 0,
///       "waypoints": [
///         {
///           "symbol": "string",
///           "type": "PLANET", // PLANETGAS GIANTMOONORBITAL STATIONJUMP GATEASTEROID
///               FIELDNEBULADEBRIS FIELDGRAVITY WELL
///           "x": 0,
///           "y": 0
///         }
///       ],
///       "factions": [
///         {
///           "symbol": "string"
///         }
///       ]
///   },
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
pub async fn get_system(system: String) -> Result<Response<Success<Data>>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.spacetraders.io/v2/systems/{}", system)).expect("url parse error");
    get(url.as_str()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_systems() {
        let res = list_systems("1".to_string()).await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
    #[tokio::test]
    async fn test_get_system() {
        let res = get_system("X1-DF55".to_string()).await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}