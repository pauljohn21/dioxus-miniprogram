//! Build commands for Dioxus Mini Program

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Build the project
pub fn build_project(
    project_dir: impl AsRef<Path>,
    release: bool,
    target_dir: Option<PathBuf>,
    worker: bool,
) -> Result<()> {
    let project_dir = project_dir.as_ref();

    tracing::info!("Building Dioxus Mini Program project: {:?}", project_dir);
    tracing::info!(
        "Worker mode: {}",
        if worker { "enabled" } else { "disabled" }
    );

    // Build WASM
    let mut cmd = std::process::Command::new("wasm-pack");
    cmd.arg("build")
        .arg("--target")
        .arg("web")
        .arg(project_dir.join("src"));

    if release {
        cmd.arg("--release");
    }

    let output_dir = target_dir.unwrap_or_else(|| project_dir.join("pkg"));
    cmd.arg("--out-dir").arg(&output_dir);

    let status = cmd.status().context("Failed to run wasm-pack")?;

    if !status.success() {
        anyhow::bail!("wasm-pack build failed with exit code: {:?}", status.code());
    }

    // Copy to mini program structure
    let miniprogram_dir = project_dir.join("miniprogram");

    std::fs::create_dir_all(&miniprogram_dir)?;

    copy_miniprogram_files(&output_dir, &miniprogram_dir, worker)?;

    tracing::info!("Build complete!");
    tracing::info!("Output directory: {:?}", miniprogram_dir);

    Ok(())
}

/// Copy and configure files for mini program structure
fn copy_miniprogram_files(wasm_dir: &Path, output_dir: &Path, worker: bool) -> Result<()> {
    // Copy WASM files
    for entry in std::fs::read_dir(wasm_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "wasm").unwrap_or(false)
            || path.extension().map(|e| e == "js").unwrap_or(false)
        {
            let dest = output_dir.join(path.file_name().unwrap());
            std::fs::copy(&path, &dest)?;
        }
    }

    // Copy glue.js if using worker mode
    if worker {
        let glue_src = PathBuf::from("packages/dioxus-miniprogram/js/glue.js");
        if glue_src.exists() {
            std::fs::create_dir_all(output_dir.join("worker"))?;
            std::fs::copy(&glue_src, output_dir.join("worker/glue.js"))?;
        }
    }

    // Create app.json if it doesn't exist
    let app_json_path = output_dir.join("app.json");
    if !app_json_path.exists() {
        let app_json = serde_json::json!({
            "pages": ["pages/index/index"],
            "window": {
                "backgroundTextStyle": "light",
                "navigationBarTextStyle": "black",
                "navigationBarTitleText": "Dioxus App",
                "navigationBarBackgroundColor": "#ffffff"
            }
        });
        std::fs::write(&app_json_path, serde_json::to_string_pretty(&app_json)?)?;
    }

    // Create pages directory
    let pages_dir = output_dir.join("pages");
    std::fs::create_dir_all(&pages_dir)?;

    let index_page_dir = pages_dir.join("index");
    std::fs::create_dir_all(&index_page_dir)?;

    // Create index.wxml
    let index_wxml = r#"<view class="container">
  <view id="dioxus-root"></view>
</view>
"#;
    std::fs::write(index_page_dir.join("index.wxml"), index_wxml)?;

    // Create index.js with Worker initialization
    let index_js = if worker {
        r#"// Initialize Worker for WASM execution
const worker = wx.createWorker('worker/glue.js')

Page({
  data: {},
  onLoad: function () {
    worker.postMessage({ type: 'init' })
    worker.onMessage((res) => {
      console.log('Worker message:', res)
    })
  },
  onUnload: function () {
    worker.terminate()
  }
});
"#
    } else {
        r#"Page({
  onLoad: function () {
    console.log('Page Load');
  }
});
"#
    };
    std::fs::write(index_page_dir.join("index.js"), index_js)?;

    // Create index.wxss
    let index_wxss = r#"page {
  height: 100%;
}
.container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20rpx;
  box-sizing: border-box;
}
"#;
    std::fs::write(index_page_dir.join("index.wxss"), index_wxss)?;

    Ok(())
}

/// Serve the project for development
pub fn serve_project(project_dir: impl AsRef<Path>, port: u16, _open: bool) -> Result<()> {
    let project_dir = project_dir.as_ref();

    tracing::info!("Starting development server on port {}", port);
    tracing::info!("Project directory: {:?}", project_dir);
    tracing::warn!("Development server is not yet implemented.");
    tracing::info!("Please use WeChat DevTools to open the miniprogram directory.");

    Ok(())
}

/// Generate a new page
pub fn generate_page(name: &str, output_dir: PathBuf) -> Result<()> {
    tracing::info!("Generating page: {}", name);

    let page_dir = output_dir.join(name);
    std::fs::create_dir_all(&page_dir)?;

    // Create page.wxml
    let page_wxml = format!(
        r#"<view class="{}">
  <view id="dioxus-root"></view>
</view>
"#,
        name
    );
    std::fs::write(page_dir.join(format!("{}.wxml", name)), page_wxml)?;

    // Create page.js
    let page_js = format!(
        r#"Page({{
  data: {{}},
  onLoad: function () {{
    console.log('{}/onLoad');
  }}
}});
"#,
        name
    );
    std::fs::write(page_dir.join(format!("{}.js", name)), page_js)?;

    // Create page.wxss
    let page_wxss = format!(
        r#".{} {{
  height: 100%;
}}
"#,
        name
    );
    std::fs::write(page_dir.join(format!("{}.wxss", name)), page_wxss)?;

    tracing::info!("Page generated at: {:?}", page_dir);

    Ok(())
}
