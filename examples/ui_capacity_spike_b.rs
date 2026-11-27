//! S1-B public resolved-target capacity probe, isolated from Pirates gameplay.
#[path = "support/ui_spike_b.rs"]
pub mod spike;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(directory) = std::env::args().nth(1) {
        spike::capture(std::path::Path::new(&directory))?;
    } else {
        spike::start()?;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
