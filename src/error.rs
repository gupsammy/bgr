use thiserror::Error;

/// Result type alias for operations that may fail with [`BgrError`].
pub type BgrResult<T> = std::result::Result<T, BgrError>;

/// Error types that can occur during background removal operations.
///
/// This enum covers errors from model inference, image I/O, mask processing,
/// and vectorization operations.
#[derive(Debug, Error)]
pub enum BgrError {
    /// ONNX Runtime inference error.
    #[error("ONNX Runtime error: {0}")]
    Ort(#[from] ort::Error),
    /// Image loading, decoding, or encoding error.
    #[error("Image processing failed: {0}")]
    Image(#[from] image::ImageError),
    /// File system I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Tensor shape mismatch or invalid dimensions.
    #[error("Invalid tensor shape: {0}")]
    Shape(#[from] ndarray::ShapeError),
    /// Vectorization or tracing operation failed.
    #[error("Tracing failed: {0}")]
    Trace(String),
    /// Alpha matte dimensions do not match the source image.
    #[error("Alpha matte size {found:?} does not match source image size {expected:?}")]
    AlphaMismatch {
        expected: (u32, u32),
        found: (u32, u32),
    },
    /// Model-related error (not found, download failed, etc.)
    #[error("{0}")]
    Model(#[from] crate::models::ModelError),
}
