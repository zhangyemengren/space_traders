use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // 多态方式
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(any: &JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_two(one: &JsValue, two: &JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_three(one: &JsValue, two: &JsValue, three: &JsValue);
}

