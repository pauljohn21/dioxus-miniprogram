# Dioxus Mini Program

[![CI](https://github.com/pauljohn21/dioxus-miniprogram/actions/workflows/ci.yml/badge.svg)](https://github.com/pauljohn21/dioxus-miniprogram/actions)
[![crates.io](https://img.shields.io/crates/v/dioxus-miniprogram.svg)](https://crates.io/crates/dioxus-miniprogram)
[![Documentation](https://docs.rs/dioxus-miniprogram/badge.svg)](https://docs.rs/dioxus-miniprogram)

A WeChat Mini Program renderer for [Dioxus](https://dioxuslabs.com/) using WXWebAssembly and Worker.

## Overview

This crate provides the ability to render Dioxus components to WeChat Mini Program. It uses WebAssembly to run Rust code and communicates with the mini program runtime via Worker threads for better performance.

## Features

- **WXWebAssembly**: Native support for WeChat's WXWebAssembly API (基础库 v2.15.0+)
- **Worker-based**: Runs WASM in a separate Worker thread for better performance
- **Reactive**: Full integration with Dioxus signals and reactivity system
- **Hot Reload**: DevTools support for rapid development
- **Type-safe**: Leverage Rust's type system for safer UI code
- **Familiar API**: Same component model as other Dioxus renderers

## Prerequisites

- Rust 1.85.0 or later
- `wasm32-unknown-unknown` target
- wasm-pack
- WeChat DevTools (基础库 v2.15.0+)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-miniprogram = "0.1"
```

## Usage

### Basic Usage

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_miniprogram::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            class: "container",
            onclick: move |_| count += 1,
            "Click me: {count}"
        }
    }
}
```

### Worker Mode (Recommended)

For production use, enable Worker mode in `Cargo.toml`:

```toml
[dependencies]
dioxus-miniprogram = { version = "0.1", features = ["worker"] }
```

## Quick Start

### 1. Create a new project

```bash
cargo install dioxus-miniprogram-cli
dx-miniprogram new my-app
cd my-app
```

### 2. Build WASM

```bash
wasm-pack build --target web
```

### 3. Open in WeChat DevTools

1. Open WeChat DevTools
2. Create a new Mini Program project
3. Point to the `miniprogram` directory

## Project Structure

```
my-miniprogram/
├── src/
│   └── lib.rs              # Your Dioxus app
├── pkg/                    # wasm-pack output
│   ├── app.wasm
│   └── app.js
├── worker/
│   └── glue.js             # Worker glue code
├── pages/
│   └── index/
│       ├── index.wxml      # Page template
│       ├── index.js        # Page logic (Worker init)
│       └── index.wxss      # Page styles
├── app.json
└── Cargo.toml
```

## WXWebAssembly Features

### Requirements

- 基础库 v2.15.0+ for Worker support
- 基础库 v2.13.0+ for basic WXWebAssembly

### Memory Management

WXWebAssembly supports `WXWebAssembly.Memory`, which is exposed to WASM modules.

### Limitations

- Global variables are not supported on iOS
- `.wasm` files must be placed outside the `worker/` directory

## Configuration

```rust
use dioxus_miniprogram::Config;

let config = Config::new()
    .with_root_id("dioxus-root")
    .with_worker(true)           // Enable Worker mode
    .with_wasm_path("pkg/app.wasm")
    .with_hot_reload(true);

dioxus_miniprogram::launch_cfg(App, config);
```

## Element Mapping

| HTML Element | Mini Program Component |
|--------------|----------------------|
| `div` | `view` |
| `span` | `text` |
| `image` | `image` |
| `input` | `input` |
| `button` | `button` |
| `scroll-view` | `scroll-view` |
| `swiper` | `swiper` |
| `navigator` | `navigator` |

## Development

### Building

```bash
# Development build
wasm-pack build --target web --dev

# Release build
wasm-pack build --target web --release
```

### Testing

```bash
wasm-pack test --chrome --headless
```

## Performance Tips

1. **Use Worker mode**: Recommended for production
2. **Enable SIMD**: From WeChat 8.0.25+
3. **Compress WASM**: Use `.wasm.br` files for smaller bundle size
4. **Code splitting**: Split large WASM files for better loading

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License or Apache-2.0 License.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) - The amazing Rust UI framework
- [WeChat Mini Program](https://developers.weixin.qq.com/miniprogram/en/) - The target platform
- [WXWebAssembly](https://developers.weixin.qq.com/miniprogram/dev/framework/performance/wasm.html) - WeChat's WASM implementation
# dioxus-miniprogram
