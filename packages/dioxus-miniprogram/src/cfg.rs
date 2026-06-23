//! Configuration options for the Dioxus Mini Program renderer

/// Configuration options for the Mini Program renderer
#[derive(Default, Clone)]
pub struct Config {
    /// The root element ID in the mini program page
    /// Defaults to "dioxus-root"
    pub root_id: Option<String>,

    /// Enable hot reload support
    /// Only works in debug builds
    pub hot_reload: bool,

    /// Use Worker mode for WASM execution
    /// When true, runs WASM in a separate Worker thread
    pub use_worker: bool,

    /// Path to the WASM file (relative to code package root)
    /// Used when use_worker is true
    pub wasm_path: Option<String>,

    /// Custom JavaScript glue code to inject
    pub custom_glue: Option<String>,
}

impl Config {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self {
            root_id: Some("dioxus-root".to_string()),
            hot_reload: false,
            use_worker: true, // Default to worker mode
            wasm_path: Some("pkg/app.wasm".to_string()),
            custom_glue: None,
        }
    }

    /// Set the root element ID
    pub fn with_root_id(mut self, root_id: &str) -> Self {
        self.root_id = Some(root_id.to_string());
        self
    }

    /// Enable or disable hot reload
    pub fn with_hot_reload(mut self, hot_reload: bool) -> Self {
        self.hot_reload = hot_reload;
        self
    }

    /// Enable or disable Worker mode
    pub fn with_worker(mut self, use_worker: bool) -> Self {
        self.use_worker = use_worker;
        self
    }

    /// Set the WASM file path
    pub fn with_wasm_path(mut self, path: &str) -> Self {
        self.wasm_path = Some(path.to_string());
        self
    }

    /// Add custom JavaScript glue code
    pub fn with_custom_glue(mut self, glue: &str) -> Self {
        self.custom_glue = Some(glue.to_string());
        self
    }
}
