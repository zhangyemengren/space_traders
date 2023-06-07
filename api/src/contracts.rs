use crate::common::{get, Response, SuccessVec};
use reqwest::{Url};

/// 列出你所有的合同
pub async fn list_contracts(
    page: String,
) -> Result<Response<SuccessVec<serde_json::Value>>, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https://api.spacetraders.io/v2/my/contracts").expect("url parse error");
    url.query_pairs_mut()
        .append_pair("limit", "20")
        .append_pair("page", page.as_str());
    get(url.as_str()).await
}
#[cfg(test)]
mod test{
    use super::*;
    #[tokio::test]
    async fn test_list_contracts() {
        let res = list_contracts("1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}
