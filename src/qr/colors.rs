//! Gradient and color calculation functions
//!
//! Provides gradient color interpolation for creating visually interesting
//! QR codes with color transitions.

use image::Rgba;

use crate::app::QrCodeApp;
use crate::types::GradientType;

/// Calculate gradient color based on pixel position
///
/// Interpolates between foreground and gradient colors based on the
/// selected gradient type and pixel position.
///
/// # Arguments
/// * `x`, `y` - Pixel coordinates in the image
/// * `width`, `height` - Total image dimensions
/// * `app` - Application state containing gradient settings
///
/// # Returns
/// RGBA color interpolated based on position and gradient type
///
/// # Gradient Types
/// - **Horizontal**: Transitions from left (fg_color) to right (gradient_color)
/// - **Vertical**: Transitions from top (fg_color) to bottom (gradient_color)
/// - **Diagonal**: Transitions from top-left to bottom-right
/// - **Radial**: Transitions from center (fg_color) outward (gradient_color)
pub fn get_gradient_color(
    x: u32, 
    y: u32, 
    width: u32, 
    height: u32, 
    app: &QrCodeApp
) -> Rgba<u8> {
    // Calculate interpolation factor (0.0 to 1.0)
    let t = match app.gradient_type {
        GradientType::Horizontal => {
            // Progress from left (0.0) to right (1.0)
            x as f32 / width as f32
        }
        GradientType::Vertical => {
            // Progress from top (0.0) to bottom (1.0)
            y as f32 / height as f32
        }
        GradientType::Diagonal => {
            // Progress from top-left (0.0) to bottom-right (1.0)
            (x + y) as f32 / (width + height) as f32
        }
        GradientType::Radial => {
            // Progress from center (0.0) to edges (1.0)
            let cx = width as f32 / 2.0;
            let cy = height as f32 / 2.0;
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let max_dist = (cx * cx + cy * cy).sqrt();
            (dist / max_dist).min(1.0) // Clamp to 1.0
        }
    };

    // Linearly interpolate between the two colors
    interpolate_rgb(
        app.fg_color, 
        app.gradient_color, 
        t
    )
}

/// Linear interpolation between two RGB colors
///
/// Blends between color1 and color2 based on factor t.
///
/// # Arguments
/// * `color1` - Starting color (RGB 0-255)
/// * `color2` - Ending color (RGB 0-255)
/// * `t` - Interpolation factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// Interpolated RGBA color (fully opaque)
///
/// # Examples
/// ```
/// // Get the midpoint color between black and white
/// let mid = interpolate_rgb([0, 0, 0], [255, 255, 255], 0.5);
/// // Result: Rgba([127, 127, 127, 255])
/// ```
fn interpolate_rgb(color1: [u8; 3], color2: [u8; 3], t: f32) -> Rgba<u8> {
    let r = lerp(color1[0], color2[0], t);
    let g = lerp(color1[1], color2[1], t);
    let b = lerp(color1[2], color2[2], t);

    Rgba([r, g, b, 255])
}

/// Linear interpolation between two u8 values
///
/// # Arguments
/// * `a` - Start value
/// * `b` - End value
/// * `t` - Interpolation factor (0.0 to 1.0)
///
/// # Returns
/// Interpolated value as u8
#[inline]
fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0, 255, 0.0), 0);
        assert_eq!(lerp(0, 255, 1.0), 255);
        assert_eq!(lerp(0, 255, 0.5), 127);
    }

    #[test]
    fn test_interpolate_rgb() {
        let black = [0, 0, 0];
        let white = [255, 255, 255];
        
        let result = interpolate_rgb(black, white, 0.5);
        assert_eq!(result[0], 127);
        assert_eq!(result[1], 127);
        assert_eq!(result[2], 127);
        assert_eq!(result[3], 255); // Alpha is always 255
    }
}
