# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                    # Build with default features (cli)
cargo build --release          # Release build
cargo test                     # Run all tests
cargo test -- --test-threads=1 # Run tests sequentially (if needed)
cargo test <module>::tests     # Run tests for a specific module (e.g., cargo test mask::tests)
cargo clippy                   # Lint
cargo fmt                      # Format
```

## Architecture

bgr is a background removal CLI and library that uses ONNX neural network models to extract foreground subjects from images. It has a dual-target design: a library crate (`bgr`) and a CLI binary (`bgr`).

### Core Processing Pipeline

The inference pipeline flows through these stages:

1. **Image Loading** (`inference.rs`) - Load RGB image with EXIF orientation correction
2. **Preprocessing** - Resize to model input dimensions, normalize with ImageNet mean/std
3. **ONNX Inference** - Run model via `ort` crate, auto-detect NCHW/NHWC layout
4. **Postprocessing** - Extract H×W matte from output tensor, resize back to original dimensions
5. **Mask Operations** (`mask.rs`) - Optional blur → threshold → dilate → fill-holes pipeline
6. **Output Generation** - Compose RGBA foreground or trace to SVG

### Key Abstractions

The library exposes a builder-style API with three main handles:

- `Bgr` - Entry point; holds model path and default settings
- `InferencedMatte` → `MatteHandle` - Raw matte from model, chainable operations
- `MaskHandle` - Processed mask, can generate foreground or SVG
- `ForegroundHandle` - Final RGBA image with transparent background

### Module Structure

- `lib.rs` - Public API: `Bgr`, `InferencedMatte`, `MatteHandle`, `MaskHandle`, `ForegroundHandle`
- `inference.rs` - ONNX session management, tensor preprocessing, model input spec detection
- `mask.rs` - Mask operations: blur, threshold, dilate, fill-holes via `MaskOperation` enum
- `config.rs` - `InferenceSettings` and `MaskProcessingOptions` structs
- `models.rs` - Model presets (`ModelPreset`), HuggingFace auto-download, path resolution
- `vectorizer/` - `MaskVectorizer` trait; `vtracer.rs` implements SVG tracing
- `foreground.rs` - RGBA composition from RGB + alpha mask
- `commands/` - CLI subcommand implementations (cut, mask, trace)
- `cli.rs` - Clap argument definitions with extensive tests for parsing behavior

### Feature Flags

- `cli` (default) - Enables CLI binary with clap, indicatif, tokio, reqwest
- `vectorizer-vtracer` - SVG tracing via vtracer/visioncortex
- `server` - HTTP API via axum (WIP)

### Model Management

Models auto-download from HuggingFace on first use. Default storage location is platform-specific:
- **macOS**: `~/Library/Application Support/bgr/models/`
- **Linux**: `~/.local/share/bgr/models/` (or `$XDG_DATA_HOME/bgr/models/`)
- **Windows**: `%APPDATA%\bgr\models\`

Supported presets: `birefnet`, `birefnet-lite`, `isnet`, `u2net`, `u2netp`, `rmbg`. Custom ONNX models can be specified by path.

### Environment Variables

- `BGR_MODEL_PATH` - Override default model path
