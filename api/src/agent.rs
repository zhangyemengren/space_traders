use crate::common::{get, Response, Success};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data{
    #[serde(rename = "accountId")]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i64,
}

/// 获取代理人的详细信息。
/// # method
/// GET
/// # request
/// # response
/// ### success
/// ```json
///{
///    "data": {
///        "accountId": "string",
///        "symbol": "string",
///        "headquarters": "string",
///        "credits": 0
///    }
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
pub async fn my_agent_details() -> Result<Response<Success<Data>>, Box<dyn std::error::Error>> {
    let resp = get("https://api.spacetraders.io/v2/my/agent").await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_agent_details() {
        let res = my_agent_details().await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}