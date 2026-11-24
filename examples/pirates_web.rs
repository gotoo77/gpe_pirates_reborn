#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    gpe_pirates_reborn::start().map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
