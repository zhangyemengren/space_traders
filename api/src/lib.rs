use std::collections::HashMap;

async fn get_ip() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let ip = get_ip().await.unwrap();
        println!("{:?}", ip);
        assert!(true);
    }
}

