//! S1 isolated capacity probe. No Pirates gameplay dependencies.
#[path = "support/ui_spike.rs"]
pub mod spike;
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(directory) = std::env::args().nth(1) {
        spike::capture_and_measure(std::path::Path::new(&directory))?;
    } else {
        spike::start()?;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
