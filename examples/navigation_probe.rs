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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn trade_state() -> String {
    use gpe_pirates_reborn::economy::{Economy, Side, TICKS_PER_DAY};

    let mut economy = Economy::default();
    let buy = economy.quote(0, 0, Side::Buy, 8).unwrap();
    economy.execute(buy).unwrap();
    for _ in 0..4 * TICKS_PER_DAY {
        economy.sea_tick();
    }
    economy.observe(1);
    let sell = economy.quote(1, 0, Side::Sell, 8).unwrap();
    economy.execute(sell).unwrap();
    format!("{economy:?}")
}

#[test]
fn native_trade_state() {
    let state = trade_state();
    println!("TRADE_STATE={state}");
    assert_eq!(state, trade_state());
}
