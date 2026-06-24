//! Project creation commands

use anyhow::Result;
use std::path::Path;

use crate::config::Config;

/// Create a new project
pub fn create_project(name: &str, template: &str) -> Result<()> {
    tracing::info!("Creating new project: {} (template: {})", name, template);

    let project_dir = Path::new(".").join(name);

    if project_dir.exists() {
        anyhow::bail!("Directory {} already exists", name);
    }

    // Create directory structure (参考 miniprogram-1)
    std::fs::create_dir_all(&project_dir)?;
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::create_dir_all(project_dir.join("pages/index"))?;
    std::fs::create_dir_all(project_dir.join("utils"))?;
    std::fs::create_dir_all(project_dir.join("pkg"))?;

    // Create Cargo.toml
    let cargo_toml = format!(
        r##"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
dioxus-miniprogram = {{ git = "https://github.com/pauljohn21/dioxus-miniprogram.git" }}
dioxus-core = "0.7"
dioxus-core-macro = "0.7"
dioxus-html = "0.7"
wasm-bindgen = "0.2"
wee_alloc = "0.4"

# Exclude from parent workspace
[workspace]

[profile.release]
opt-level = "s"
lto = true
"##,
        name
    );
    std::fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    // Create dioxus-miniprogram.toml
    let config = Config::default();
    config.save(project_dir.join("dioxus-miniprogram.toml"))?;

    // Create src/lib.rs
    let lib_rs = r##"//! My Dioxus Mini Program App

use dioxus_core::*;
use dioxus_core_macro::{rsx, component};

// Wrapper module for dioxus-html elements
// The rsx! macro expects elements to be at dioxus_elements::*
mod dioxus_elements {
    pub use dioxus_html::*;
    pub mod elements {
        pub use dioxus_html::*;
    }
    pub mod events {
        pub use dioxus_html::events::*;
    }
}

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() {
    dioxus_miniprogram::launch::launch(App);
}

fn App() -> dioxus_core::Element {
    rsx! {
        view {
            class: "container",
            "Hello, Dioxus Mini Program!"
        }
    }
}
"##;
    std::fs::write(project_dir.join("src/lib.rs"), lib_rs)?;

    // Create app.js (参考 miniprogram-1)
    let app_js = r##"// app.js
App({
  onLaunch() {
    console.log('Dioxus Mini Program launched')
  },
  globalData: {
    userInfo: null
  }
})
"##;
    std::fs::write(project_dir.join("app.js"), app_js)?;

    // Create app.json (参考 miniprogram-1)
    let app_json = r##"{
  "pages": [
    "pages/index/index"
  ],
  "window": {
    "navigationBarTextStyle": "black",
    "navigationBarTitleText": "Dioxus App",
    "navigationBarBackgroundColor": "#ffffff"
  },
  "style": "v2",
  "componentFramework": "glass-easel",
  "sitemapLocation": "sitemap.json",
  "lazyCodeLoading": "requiredComponents"
}
"##;
    std::fs::write(project_dir.join("app.json"), app_json)?;

    // Create app.wxss
    let app_wxss = r##"/**app.wxss**/
page {
  background-color: #f8f8f8;
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
}

.container {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20rpx;
  box-sizing: border-box;
}
"##;
    std::fs::write(project_dir.join("app.wxss"), app_wxss)?;

    // Create sitemap.json
    let sitemap_json = r##"{
  "desc": "关于本文件的更多信息，请参考文档 https://developers.weixin.qq.com/miniprogram/dev/framework/sitemap.html",
  "rules": [{
    "action": "allow",
    "page": "*"
  }]
}
"##;
    std::fs::write(project_dir.join("sitemap.json"), sitemap_json)?;

    // Create project.config.json
    let project_config_json = serde_json::json!({
        "miniprogramRoot": "./",
        "projectname": name,
        "description": "Dioxus Mini Program",
        "appid": "touristappid",
        "setting": {
            "urlCheck": false,
            "es6": true,
            "enhance": true,
            "postcss": true,
            "minified": true
        },
        "compileType": "miniprogram",
        "condition": {}
    });
    std::fs::write(
        project_dir.join("project.config.json"),
        serde_json::to_string_pretty(&project_config_json)?,
    )?;

    // Create utils/wasm.js (参考 miniprogram-1 的 utils/wasm.js)
    let wasm_js = r##"/**
 * Dioxus WASM Wrapper for Mini Program
 */

let wasm = null;
let WASM_VECTOR_LEN = 0;
let cachedTextDecoder = null;
let cachedTextEncoder = null;
let cachedUint8ArrayMemory0 = null;

const WASM_PATH = '/pkg/app_bg.wasm';

function initMemory() {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    cachedTextEncoder = new TextEncoder();
}

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr >>> 0, (ptr >>> 0) + len));
}

export async function initWasm() {
    if (wasm) return wasm;

    try {
        console.log('[Dioxus] Loading WASM...');
        
        if (typeof WXWebAssembly === 'undefined') {
            throw new Error('WXWebAssembly not available. Need base library v2.15.0+');
        }

        const { instance } = await WXWebAssembly.instantiate(WASM_PATH, {
            "./app_bg.js": {
                __wbindgen_init_externref_table: function() {
                    const table = instance.exports.__wbindgen_externrefs;
                    const offset = table.grow(4);
                    table.set(0, undefined);
                    table.set(offset + 0, undefined);
                    table.set(offset + 1, null);
                    table.set(offset + 2, true);
                    table.set(offset + 3, false);
                }
            }
        });
        
        wasm = instance.exports;
        
        if (typeof wasm.__wbindgen_start === 'function') {
            wasm.__wbindgen_start();
        }
        
        initMemory();
        console.log('[Dioxus] WASM loaded');
        return wasm;
    } catch (error) {
        console.error('[Dioxus] WASM load failed:', error);
        throw error;
    }
}

export async function runDioxus() {
    await initWasm();
    if (wasm.run) wasm.run();
}

export function isWasmReady() {
    return wasm !== null;
}
"##;
    std::fs::write(project_dir.join("utils/wasm.js"), wasm_js)?;

    // Create pages/index/index.wxml
    let index_wxml = r##"<view class="container">
  <view id="dioxus-root">
    <text>{{message}}</text>
  </view>
</view>
"##;
    std::fs::write(project_dir.join("pages/index/index.wxml"), index_wxml)?;

    // Create pages/index/index.js (参考 miniprogram-1)
    let index_js = r##"import { initWasm, runDioxus, isWasmReady } from '../../utils/wasm.js';

Page({
  data: {
    message: 'Loading...',
    wasmLoading: true
  },

  async onLoad() {
    try {
      await initWasm();
      this.setData({ wasmLoading: false, message: 'WASM Ready!' });
      
      // Run Dioxus app
      runDioxus();
      
      // Poll for mutations
      this.startMutationPolling();
    } catch (error) {
      this.setData({ 
        message: 'Error: ' + error.message,
        wasmLoading: false 
      });
    }
  },

  startMutationPolling() {
    // Poll for DOM mutations from WASM
    setInterval(() => {
      if (isWasmReady()) {
        // Get mutations and apply to view
        // This will be implemented when DOM bridge is complete
      }
    }, 16); // ~60fps
  },

  onUnload() {
    // Cleanup
  }
});
"##;
    std::fs::write(project_dir.join("pages/index/index.js"), index_js)?;

    // Create pages/index/index.json
    let index_json = r##"{
  "usingComponents": {}
}
"##;
    std::fs::write(project_dir.join("pages/index/index.json"), index_json)?;

    // Create pages/index/index.wxss
    let index_wxss = r##"page {
  height: 100%;
}

.container {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 40rpx;
}

#dioxus-root {
  width: 100%;
}
"##;
    std::fs::write(project_dir.join("pages/index/index.wxss"), index_wxss)?;

    tracing::info!("Project created at: {:?}", project_dir);
    tracing::info!("");
    tracing::info!("Next steps:");
    tracing::info!("  cd {}", name);
    tracing::info!("  wasm-pack build --target web --out-dir pkg");
    tracing::info!("  # Open in WeChat DevTools");

    Ok(())
}

/// Initialize a project in an existing directory
pub fn init_project(dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();

    tracing::info!("Initializing project in: {:?}", dir);

    // Check for Cargo.toml
    let cargo_path = dir.join("Cargo.toml");
    if !cargo_path.exists() {
        anyhow::bail!("No Cargo.toml found. Please run this command in a Rust project.");
    }

    // Create config file
    let config_path = dir.join("dioxus-miniprogram.toml");
    if config_path.exists() {
        tracing::warn!("Config file already exists, skipping...");
    } else {
        let config = Config::default();
        config.save(&config_path)?;
        tracing::info!("Created: {:?}", config_path);
    }

    // Create utils directory
    let utils_dir = dir.join("utils");
    std::fs::create_dir_all(&utils_dir)?;

    // Create utils/wasm.js if not exists
    let wasm_js_path = utils_dir.join("wasm.js");
    if !wasm_js_path.exists() {
        let wasm_js = r##"let wasm = null;

export async function initWasm() {
    if (wasm) return wasm;
    
    const { instance } = await WXWebAssembly.instantiate('/pkg/app_bg.wasm', {});
    wasm = instance.exports;
    
    if (wasm.__wbindgen_start) wasm.__wbindgen_start();
    return wasm;
}

export async function runDioxus() {
    await initWasm();
    if (wasm.run) wasm.run();
}
"##;
        std::fs::write(&wasm_js_path, wasm_js)?;
        tracing::info!("Created: {:?}", wasm_js_path);
    }

    // Create app.js if not exists
    let app_js_path = dir.join("app.js");
    if !app_js_path.exists() {
        let app_js = "App({ onLaunch() { console.log('App launched') } })\n";
        std::fs::write(&app_js_path, app_js)?;
        tracing::info!("Created: {:?}", app_js_path);
    }

    // Create app.json if not exists
    let app_json_path = dir.join("app.json");
    if !app_json_path.exists() {
        let app_json = serde_json::json!({
            "pages": ["pages/index/index"],
            "window": {
                "navigationBarTitleText": "Dioxus App"
            },
            "sitemapLocation": "sitemap.json"
        });
        std::fs::write(&app_json_path, serde_json::to_string_pretty(&app_json)?)?;
        tracing::info!("Created: {:?}", app_json_path);
    }

    // Create pages directory
    let pages_dir = dir.join("pages/index");
    std::fs::create_dir_all(&pages_dir)?;

    // Create page files
    if !pages_dir.join("index.wxml").exists() {
        std::fs::write(
            pages_dir.join("index.wxml"),
            "<view id=\"dioxus-root\"></view>\n",
        )?;
        tracing::info!("Created: {:?}", pages_dir.join("index.wxml"));
    }

    if !pages_dir.join("index.js").exists() {
        let index_js = r##"import { initWasm, runDioxus } from '../../utils/wasm.js';
Page({
  async onLoad() {
    await initWasm();
    runDioxus();
  }
});
"##;
        std::fs::write(pages_dir.join("index.js"), index_js)?;
        tracing::info!("Created: {:?}", pages_dir.join("index.js"));
    }

    tracing::info!("Project initialized!");

    Ok(())
}
