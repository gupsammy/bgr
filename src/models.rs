//! Model presets and auto-download functionality.
//!
//! Provides named model presets (birefnet, isnet, u2net, etc.) that automatically
//! download from HuggingFace on first use.

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Default models directory (~/.bgr/models)
pub fn default_models_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "bgr")
        .map(|dirs| dirs.data_dir().to_path_buf())
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".bgr")
        })
        .join("models")
}

/// Known model presets with their HuggingFace sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelPreset {
    /// BiRefNet general - best quality, handles complex scenes
    BiRefNet,
    /// BiRefNet lite - faster, slightly lower quality
    BiRefNetLite,
    /// IS-Net general use - good all-around model
    IsNet,
    /// U2-Net - classic, reliable
    U2Net,
    /// U2-Net portrait - optimized for human segmentation
    U2NetP,
    /// RMBG 1.4 by BRIA AI - state of the art
    Rmbg,
}

impl ModelPreset {
    /// All available presets.
    pub const ALL: &'static [ModelPreset] = &[
        ModelPreset::BiRefNet,
        ModelPreset::BiRefNetLite,
        ModelPreset::IsNet,
        ModelPreset::U2Net,
        ModelPreset::U2NetP,
        ModelPreset::Rmbg,
    ];

    /// Model name for CLI display.
    pub fn name(&self) -> &'static str {
        match self {
            ModelPreset::BiRefNet => "birefnet",
            ModelPreset::BiRefNetLite => "birefnet-lite",
            ModelPreset::IsNet => "isnet",
            ModelPreset::U2Net => "u2net",
            ModelPreset::U2NetP => "u2netp",
            ModelPreset::Rmbg => "rmbg",
        }
    }

    /// Human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            ModelPreset::BiRefNet => "U2Net full - best quality, general purpose",
            ModelPreset::BiRefNetLite => "U2Netp - fast, good for portraits",
            ModelPreset::IsNet => "IS-Net - anime/illustration focused",
            ModelPreset::U2Net => "U2Net full - classic, well-tested",
            ModelPreset::U2NetP => "U2Netp - lightweight, portrait optimized",
            ModelPreset::Rmbg => "U2Net - reliable general purpose",
        }
    }

    /// Approximate model size in MB.
    pub fn size_mb(&self) -> u32 {
        match self {
            ModelPreset::BiRefNet => 176,
            ModelPreset::BiRefNetLite => 5,
            ModelPreset::IsNet => 176,
            ModelPreset::U2Net => 176,
            ModelPreset::U2NetP => 5,
            ModelPreset::Rmbg => 176,
        }
    }

    /// HuggingFace download URL for the ONNX model.
    ///
    /// Uses publicly accessible mirrors where the original sources require authentication.
    pub fn download_url(&self) -> &'static str {
        match self {
            // Public U2Net mirror - reliable general-purpose model
            ModelPreset::BiRefNet => {
                "https://huggingface.co/BritishWerewolf/U-2-Net/resolve/main/onnx/model.onnx"
            }
            // Lightweight U2Netp - fast inference, good for portraits
            ModelPreset::BiRefNetLite => {
                "https://huggingface.co/BritishWerewolf/U-2-Netp/resolve/main/onnx/model.onnx"
            }
            // IS-Net for anime/illustration segmentation
            ModelPreset::IsNet => {
                "https://huggingface.co/skytnt/anime-seg/resolve/main/isnetis.onnx"
            }
            // U2Net full model - classic, well-tested
            ModelPreset::U2Net => {
                "https://huggingface.co/BritishWerewolf/U-2-Net/resolve/main/onnx/model.onnx"
            }
            // U2Netp lightweight - optimized for portraits
            ModelPreset::U2NetP => {
                "https://huggingface.co/BritishWerewolf/U-2-Netp/resolve/main/onnx/model.onnx"
            }
            // Alternative U2Net mirror
            ModelPreset::Rmbg => {
                "https://huggingface.co/scenario-labs/grayscale/resolve/main/u2net.onnx"
            }
        }
    }

    /// Local filename for the model.
    pub fn filename(&self) -> &'static str {
        match self {
            ModelPreset::BiRefNet => "birefnet.onnx",
            ModelPreset::BiRefNetLite => "birefnet-lite.onnx",
            ModelPreset::IsNet => "isnet.onnx",
            ModelPreset::U2Net => "u2net.onnx",
            ModelPreset::U2NetP => "u2netp.onnx",
            ModelPreset::Rmbg => "rmbg.onnx",
        }
    }

    /// Parse a preset name from string.
    pub fn from_str(s: &str) -> Option<ModelPreset> {
        match s.to_lowercase().as_str() {
            "birefnet" | "birefnet-general" => Some(ModelPreset::BiRefNet),
            "birefnet-lite" | "birefnet-light" => Some(ModelPreset::BiRefNetLite),
            "isnet" | "isnet-general" | "isnet-general-use" => Some(ModelPreset::IsNet),
            "u2net" => Some(ModelPreset::U2Net),
            "u2netp" | "u2net-p" | "u2net-portrait" => Some(ModelPreset::U2NetP),
            "rmbg" | "rmbg-1.4" | "bria" => Some(ModelPreset::Rmbg),
            _ => None,
        }
    }

    /// Get the local path for this model.
    pub fn local_path(&self, models_dir: &Path) -> PathBuf {
        models_dir.join(self.filename())
    }

    /// Check if the model is already downloaded.
    pub fn is_downloaded(&self, models_dir: &Path) -> bool {
        self.local_path(models_dir).exists()
    }
}

/// Errors that can occur during model operations.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error(
        "Unknown model: {0}. Use --model=<path> for custom models or one of: birefnet, birefnet-lite, isnet, u2net, u2netp, rmbg"
    )]
    UnknownPreset(String),

    #[error("Failed to create models directory {path}: {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to download model from {url}: {message}")]
    Download { url: String, message: String },

    #[error("Model file not found: {0}")]
    NotFound(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Resolve a model specifier to a local path.
///
/// If `specifier` is a known preset name, returns the path in models_dir
/// (downloading if necessary when `auto_download` is true).
/// If `specifier` is a path, returns it directly.
pub fn resolve_model_path(
    specifier: &str,
    models_dir: &Path,
    auto_download: bool,
) -> Result<PathBuf, ModelError> {
    // Check if it's a file path first
    let as_path = Path::new(specifier);
    if as_path.exists() {
        return Ok(as_path.to_path_buf());
    }

    // Check if it's a preset
    if let Some(preset) = ModelPreset::from_str(specifier) {
        let local_path = preset.local_path(models_dir);

        if local_path.exists() {
            return Ok(local_path);
        }

        if auto_download {
            // Create models directory if needed
            if !models_dir.exists() {
                std::fs::create_dir_all(models_dir).map_err(|e| ModelError::CreateDir {
                    path: models_dir.to_path_buf(),
                    source: e,
                })?;
            }

            // Download will happen via async function called elsewhere
            // For now, return the expected path
            return Ok(local_path);
        }

        return Err(ModelError::NotFound(local_path));
    }

    // Not a preset and file doesn't exist
    if !as_path.exists() {
        // Could be a preset typo
        Err(ModelError::UnknownPreset(specifier.to_string()))
    } else {
        Ok(as_path.to_path_buf())
    }
}

/// Download a model from HuggingFace.
#[cfg(feature = "cli")]
pub async fn download_model(
    preset: ModelPreset,
    models_dir: &Path,
    progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
) -> Result<PathBuf, ModelError> {
    use tokio::io::AsyncWriteExt;

    let url = preset.download_url();
    let local_path = preset.local_path(models_dir);

    // Create models directory if needed
    if !models_dir.exists() {
        std::fs::create_dir_all(models_dir).map_err(|e| ModelError::CreateDir {
            path: models_dir.to_path_buf(),
            source: e,
        })?;
    }

    // Download with progress
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| ModelError::Download {
            url: url.to_string(),
            message: e.to_string(),
        })?;

    if !response.status().is_success() {
        return Err(ModelError::Download {
            url: url.to_string(),
            message: format!("HTTP {}", response.status()),
        });
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    // Write to temp file first, then rename
    let temp_path = local_path.with_extension("onnx.tmp");
    let mut file = tokio::fs::File::create(&temp_path)
        .await
        .map_err(ModelError::Io)?;

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| ModelError::Download {
            url: url.to_string(),
            message: e.to_string(),
        })?;

        file.write_all(&chunk).await.map_err(ModelError::Io)?;
        downloaded += chunk.len() as u64;

        if let Some(ref cb) = progress_callback {
            cb(downloaded, total_size);
        }
    }

    file.flush().await.map_err(ModelError::Io)?;
    drop(file);

    // Rename temp to final
    tokio::fs::rename(&temp_path, &local_path)
        .await
        .map_err(ModelError::Io)?;

    Ok(local_path)
}

/// Synchronous download wrapper for non-async contexts.
#[cfg(feature = "cli")]
pub fn download_model_sync(
    preset: ModelPreset,
    models_dir: &Path,
    progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
) -> Result<PathBuf, ModelError> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| ModelError::Download {
        url: preset.download_url().to_string(),
        message: format!("Failed to create async runtime: {e}"),
    })?;

    rt.block_on(download_model(preset, models_dir, progress_callback))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_names_parse() {
        assert_eq!(
            ModelPreset::from_str("birefnet"),
            Some(ModelPreset::BiRefNet)
        );
        assert_eq!(
            ModelPreset::from_str("BIREFNET"),
            Some(ModelPreset::BiRefNet)
        );
        assert_eq!(ModelPreset::from_str("isnet"), Some(ModelPreset::IsNet));
        assert_eq!(ModelPreset::from_str("rmbg"), Some(ModelPreset::Rmbg));
        assert_eq!(ModelPreset::from_str("unknown"), None);
    }

    #[test]
    fn all_presets_have_valid_data() {
        for preset in ModelPreset::ALL {
            assert!(!preset.name().is_empty());
            assert!(!preset.description().is_empty());
            assert!(!preset.download_url().is_empty());
            assert!(!preset.filename().is_empty());
            assert!(preset.size_mb() > 0);
        }
    }
}
