use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // js 多态绑定test
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
pub fn js_console() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}

pub async fn run() -> Result<JsValue, JsValue> {
    let res = reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let text = res.text().await?;
    let branch_info: serde_json::Value = serde_json::from_str(&text).unwrap();

    Ok(JsValue::from_str(&branch_info.as_str().unwrap()))
}