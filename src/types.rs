//! Type definitions for QR code generation and styling
//!
//! This module contains all the enums and data structures used throughout
//! the application for QR code customization, including:
//! - Error correction levels
//! - Visual styling options (modules, eyes, gradients)
//! - UI navigation types
//! - Color presets

use serde::{Serialize, Deserialize};
use qrcode::EcLevel;

/// QR code error correction level
///
/// Higher levels allow the QR code to be decoded even with damage or obstruction,
/// but result in larger QR codes with more modules.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ErrorCorrectionLevel {
    /// Low: ~7% error correction - smallest QR codes
    Low,
    /// Medium: ~15% error correction - balanced (recommended)
    Medium,
    /// Quartile: ~25% error correction - good for styling
    Quartile,
    /// High: ~30% error correction - maximum resilience
    High,
}

impl ErrorCorrectionLevel {
    /// Convert to the qrcode crate's EcLevel type
    ///
    /// # Returns
    /// The corresponding EcLevel for QR code generation
    pub fn to_ec_level(&self) -> EcLevel {
        match self {
            ErrorCorrectionLevel::Low => EcLevel::L,
            ErrorCorrectionLevel::Medium => EcLevel::M,
            ErrorCorrectionLevel::Quartile => EcLevel::Q,
            ErrorCorrectionLevel::High => EcLevel::H,
        }
    }
}

/// Visual style for QR code data modules
///
/// Different module styles create distinct visual appearances while
/// maintaining QR code scannability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ModuleStyle {
    /// Traditional square modules - most reliable for scanning
    Square,
    /// Circular modules - smooth, modern appearance
    Circle,
    /// Rounded square modules - softer edges with adjustable radius
    RoundedSquare,
    /// Small dot-style modules - minimalist, artistic look
    Dots,
}

/// Tab selection for UI navigation
///
/// Organizes controls into logical groups for better user experience.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TabSelection {
    /// Basic settings: content, size, error correction
    Basic,
    /// Visual styling: colors, gradients, module shapes
    Style,
    /// Advanced options: opacity, fine-tuning
    Advanced,
    /// Image operations: logos, backgrounds
    Images,
}

/// Gradient direction and style
///
/// Defines how colors blend across the QR code when gradients are enabled.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GradientType {
    /// Linear gradient from left to right
    Horizontal,
    /// Linear gradient from top to bottom
    Vertical,
    /// Diagonal gradient from top-left to bottom-right
    Diagonal,
    /// Radial gradient from center outward
    Radial,
}

/// Eye (finder pattern) visual style
///
/// The three corner squares that help scanners locate the QR code.
/// Custom styles can make QR codes more visually distinctive.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EyeStyle {
    /// Standard square eyes - traditional QR appearance
    Standard,
    /// Circular eyes - smooth, modern look
    Circle,
    /// Rounded square eyes - softened corners
    RoundedSquare,
    /// Flower-shaped eyes - decorative, artistic
    Flower,
    /// Diamond-shaped eyes - geometric, distinctive
    Diamond,
}

/// Predefined color scheme for quick styling
///
/// Provides professionally designed color combinations for instant use.
pub struct ColorPreset {
    /// Display name for the preset
    pub name: &'static str,
    /// Foreground (dark modules) RGB color
    pub fg: [u8; 3],
    /// Background (light areas) RGB color
    pub bg: [u8; 3],
}

/// Collection of built-in color presets
///
/// These presets offer a variety of professionally designed color schemes
/// suitable for different use cases and aesthetic preferences.
pub const COLOR_PRESETS: &[ColorPreset] = &[
    ColorPreset {
        name: "Classic",
        fg: [0, 0, 0],
        bg: [255, 255, 255],
    },
    ColorPreset {
        name: "Ocean",
        fg: [0, 119, 182],
        bg: [224, 247, 250],
    },
    ColorPreset {
        name: "Sunset",
        fg: [255, 87, 34],
        bg: [255, 243, 224],
    },
    ColorPreset {
        name: "Forest",
        fg: [27, 94, 32],
        bg: [232, 245, 233],
    },
    ColorPreset {
        name: "Purple",
        fg: [123, 31, 162],
        bg: [243, 229, 245],
    },
    ColorPreset {
        name: "Rose",
        fg: [194, 24, 91],
        bg: [252, 228, 236],
    },
    ColorPreset {
        name: "Night",
        fg: [255, 255, 255],
        bg: [33, 33, 33],
    },
    ColorPreset {
        name: "Cyber",
        fg: [0, 255, 255],
        bg: [10, 10, 40],
    },
];
