//! QR code generation and rendering
//!
//! This module is responsible for all QR code generation logic,
//! including module drawing, color gradients, and image integration.

pub mod generator;
pub mod drawing;
pub mod colors;
pub mod images;

// Re-export main generation function for convenience
pub use generator::generate_qr_image;
