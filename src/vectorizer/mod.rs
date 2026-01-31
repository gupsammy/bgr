use image::GrayImage;

use crate::BgrResult;

/// A trait representing an algorithm that can turn a mask into a vector representation.
pub trait MaskVectorizer {
    type Options;
    type Output;

    fn vectorize(&self, mask: &GrayImage, options: &Self::Options) -> BgrResult<Self::Output>;
}

#[cfg(feature = "vectorizer-vtracer")]
pub mod vtracer;
