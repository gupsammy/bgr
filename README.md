# bgr — Background Removal CLI

Fast, high-quality background removal powered by AI models.

```bash
bgr cut photo.jpg                    # → photo-nobg.png
bgr cut photo.jpg -m birefnet        # Use BiRefNet model
bgr cut photo.jpg --blur=2           # Soft feathered edges
```

## Features

- **Multiple AI Models** — BiRefNet, IS-Net, U2-Net, RMBG (auto-download from HuggingFace)
- **Flexible Mask Processing** — blur, threshold, dilation, hole-filling
- **SVG Tracing** — Convert masks to vector outlines
- **High Quality** — Powered by ONNX Runtime with GPU acceleration

## Installation

```bash
cargo install bgr
```

Or build from source:
```bash
git clone https://github.com/gupsammy/bgr
cd bgr
cargo install --path .
```

## Usage

### Remove Background

```bash
# Basic usage (auto-downloads model on first run)
bgr cut input.jpg -o output.png

# Specify model
bgr cut input.jpg -m birefnet         # Best quality
bgr cut input.jpg -m isnet            # General purpose
bgr cut input.jpg -m rmbg             # BRIA state-of-art

# Soft/feathered edges
bgr cut input.jpg --blur -o soft.png

# Hard edges with processing
bgr cut input.jpg --blur --dilate=5 -o hard.png
```

### Export Mask Only

```bash
bgr mask input.jpg                    # → input-matte.png (grayscale)
bgr mask input.jpg --binary           # → input-mask.png (black/white)
```

### Generate SVG Outline

```bash
bgr trace input.jpg                   # → input.svg
bgr trace input.jpg --dilate=50 --fill-holes  # Sticker-style
```

## Model Management

Models are auto-downloaded on first use to platform-specific directories:
- **macOS**: `~/Library/Application Support/bgr/models/`
- **Linux**: `~/.local/share/bgr/models/`
- **Windows**: `%APPDATA%\bgr\models\`

### Available Models

| Model | Size | Best For |
|-------|------|----------|
| `birefnet` | 176MB | Complex scenes, hair/fur |
| `birefnet-lite` | 5MB | Faster, good for portraits |
| `isnet` | 176MB | General purpose |
| `u2net` | 176MB | Classic, reliable |
| `rmbg` | 176MB | BRIA AI state-of-art |

## Configuration

### Environment Variables

```bash
BGR_MODEL_PATH=/path/to/model.onnx    # Custom model path
```

### Custom ONNX Models

Use any compatible ONNX model:
```bash
bgr cut input.jpg -m /path/to/custom.onnx
```

## Mask Processing Options

| Flag | Description |
|------|-------------|
| `--blur [sigma]` | Gaussian blur (default σ=6.0) |
| `--mask-threshold <0-255>` | Binary threshold (default 120) |
| `--binary` | Force binary mask output |
| `--dilate [radius]` | Expand mask (default r=5.0) |
| `--fill-holes` | Fill enclosed holes |

## Credits

This project is a fork of [outline](https://github.com/wyh2001/outline) by Yihang Wang,
extended with model auto-download, batch processing, and additional features.

Licensed under MIT. See [LICENSE](LICENSE) for details.
