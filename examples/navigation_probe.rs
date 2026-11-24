#[path = "../tests/support/replay.rs"]
mod replay;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn reference_state() -> String {
    replay::reference_state()
}

#[test]
fn native_reference_state() {
    let state = reference_state();
    println!("REFERENCE_STATE={state}");
    assert_eq!(state, reference_state());
}
