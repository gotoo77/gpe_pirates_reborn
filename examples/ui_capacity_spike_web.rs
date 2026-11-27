#[path = "support/ui_spike.rs"]
pub mod spike;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    spike::start().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
#[cfg(not(target_arch = "wasm32"))]
pub fn start() -> Result<(), gotoo_pixel_engine::EngineError> {
    spike::start()
}
