//! DevTools support for hot reload
//!
//! This module provides integration with Dioxus DevTools for hot reload
//! support during development.

use crate::cfg::Config;

/// Initialize devtools (no-op for WASM)
#[cfg(target_family = "wasm")]
pub fn init(_config: &Config) {}

/// Initialize devtools for non-WASM targets
#[cfg(not(target_family = "wasm"))]
pub fn init(_config: &Config) {
    #[cfg(feature = "devtools")]
    {
        dioxus_devtools::connect(|_event| {
            // Handle hot reload events
        });
    }
}
