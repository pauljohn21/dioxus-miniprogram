//! Launch functions for Dioxus Mini Program
//!
//! These functions are used to start a Dioxus application with the Mini Program renderer.

use crate::cfg::Config;
use dioxus_core::{RenderError, VNode, VirtualDom};

/// Launch the dioxus app with the default configuration.
///
/// # Example
///
/// ```rust, ignore
/// fn main() {
///     dioxus_miniprogram::launch(App);
/// }
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         div { "Hello, World!" }
///     }
/// }
/// ```
pub fn launch(app: fn() -> Result<VNode, RenderError>) {
    wasm_bindgen_futures::spawn_local(async move {
        let vdom = VirtualDom::new(app);
        let config = Config::new();
        crate::run(vdom, config).await;
    });
}

/// Launch the dioxus app with a custom configuration.
///
/// # Example
///
/// ```rust, ignore
/// fn main() {
///     let config = dioxus_miniprogram::Config::new()
///         .with_root_id("app")
///         .with_worker(true);
///     dioxus_miniprogram::launch_cfg(App, config);
/// }
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         div { "Hello, World!" }
///     }
/// }
/// ```
pub fn launch_cfg(app: fn() -> Result<VNode, RenderError>, config: Config) {
    wasm_bindgen_futures::spawn_local(async move {
        let vdom = VirtualDom::new(app);
        crate::run(vdom, config).await;
    });
}

/// Launch the dioxus app using Worker mode with WXWebAssembly
///
/// This function creates a Worker that runs the WASM code and communicates
/// with the main thread via postMessage. This is the recommended mode for
/// production use as it provides better performance.
pub async fn launch_with_worker(_app: fn() -> Result<VNode, RenderError>) {
    let config = Config::new();
    crate::run_with_worker(config).await.unwrap();
}
