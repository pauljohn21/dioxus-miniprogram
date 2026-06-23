#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]
#![deny(missing_docs)]

//! # Dioxus Mini Program
//!
//! WeChat Mini Program renderer for Dioxus using WXWebAssembly and Worker.
//!
//! This crate provides the ability to render Dioxus components to WeChat Mini Program.
//! It uses WebAssembly to run Rust code and communicates with the mini program runtime via Worker.
//!
//! ## Features
//!
//! - **WXWebAssembly**: Native support for WeChat's WXWebAssembly API
//! - **Worker-based**: Runs WASM in a separate Worker thread for better performance
//! - **Reactive**: Full integration with Dioxus signals and reactivity system
//! - **DevTools**: Hot reload support for rapid development
//! - **Type-safe**: Leverage Rust's type system for safer UI code
//!
//! ## Usage
//!
//! ```rust, ignore
//! use dioxus::prelude::*;
//!
//! fn main() {
//!     dioxus_miniprogram::launch(App);
//! }
//!
//! #[component]
//! fn App() -> Element {
//!     let mut count = use_signal(|| 0);
//!     
//!     rsx! {
//!         div {
//!             onclick: move |_| count += 1,
//!             "Click me: {count}"
//!         }
//!     }
//! }
//! ```
//!
//! ## Setup
//!
//! 1. Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! dioxus-miniprogram = "0.1"
//! ```
//!
//! 2. Add the JavaScript glue code to your project:
//!    - Copy `js/glue.js` to your mini program's `worker/glue.js`
//!    - Ensure your `.wasm` file is placed outside the `worker/` directory

pub use crate::cfg::Config;
use crate::dom::MiniProgramDom;
use dioxus_core::VirtualDom;

mod cfg;
mod dom;
pub mod launch;
mod worker;

#[cfg(feature = "devtools")]
mod devtools;

/// Runs the app as a future that can be scheduled around the main thread.
///
/// Uses Worker-based execution with WXWebAssembly for better performance.
///
/// # Example
///
/// ```ignore, rust
/// let app_fut = dioxus_miniprogram::run_with_props(App, RootProps { name: String::from("foo") });
/// wasm_bindgen_futures::spawn_local(app_fut);
/// ```
pub async fn run(mut virtual_dom: VirtualDom, config: Config) -> ! {
    let runtime = virtual_dom.runtime();

    // Create the DOM bridge
    let mut mp_dom = MiniProgramDom::new(config, runtime);

    // Initial render
    virtual_dom.rebuild(&mut mp_dom);

    loop {
        // Wait for work from the virtual dom
        virtual_dom.wait_for_work().await;

        // Run the virtualdom work phase
        virtual_dom.render_immediate(&mut mp_dom);
    }
}

/// Run the app using Worker mode with WXWebAssembly
///
/// This function creates a Worker that runs the WASM code and communicates
/// with the main thread via postMessage.
pub async fn run_with_worker(_config: Config) -> Result<(), JsValue> {
    // Create the worker
    let worker_js = include_str!("../js/glue.js");

    // Create a blob URL for the worker script
    let js_value = wasm_bindgen::JsValue::from(worker_js);
    let blob = web_sys::Blob::new_with_str_sequence(&js_value)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob)?;

    // Create the worker
    let worker = web_sys::Worker::new(&url)?;

    // Handle messages from worker
    let on_message = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        let data = event.data();
        tracing::debug!("Worker message: {:?}", data);
    }) as Box<dyn FnMut(_)>);
    worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    // Keep the closure alive
    on_message.forget();

    Ok(())
}
