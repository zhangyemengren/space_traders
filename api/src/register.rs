use crate::common::{post, Response, Success};
use reqwest::Url;
use serde_json::{json, Value};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
    data: Value,
    token: String,
}
/// 注册新的账户并将返回的令牌存储在本地
pub async fn register() -> Result<Response<Success<Data>>, Box<dyn std::error::Error>> {
    let url = Url::parse("https://api.spacetraders.io/v2/register")?;
    let file_path = ".env";
    let mut file = File::create(file_path)?;
    let res: Response<Success<Data>> = post(
        url.as_str(),
        json!({
            "faction": "COSMIC",
            "symbol": "zhangyemengren",
            "email": "6245937581@qq.com"
        }),
    )
    .await?;
    if let Response::Success(ref s) = res {
        file.write_all(format!("token={}", s.data.token).as_bytes())?;
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register() {
        let res = register().await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}
