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
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_obj(obj: &JsValue);
}
pub fn js_console() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
    log_obj(&JsValue::from("{a: 1, b: 2}"));
}
