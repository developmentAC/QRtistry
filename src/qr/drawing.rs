//! Module and eye drawing functions
//!
//! Handles the pixel-level rendering of QR code modules in various styles:
//! - Square (traditional)
//! - Circle (smooth, modern)
//! - Rounded Square (soft edges)
//! - Dots (minimalist)
//!
//! Also handles special eye (finder pattern) styling.

use image::{Rgba, RgbaImage};

use crate::app::QrCodeApp;
use crate::types::{ModuleStyle, EyeStyle};
use crate::qr::colors;

/// Draw a data module (non-eye module) with the selected style
///
/// Applies the current module style setting and gradient colors if enabled.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `app` - Application state for style settings
/// * `x`, `y` - Module coordinates in QR matrix
/// * `px`, `py` - Pixel coordinates in image
/// * `size` - Size of module in pixels
pub fn draw_data_module(
    image: &mut RgbaImage, 
    app: &QrCodeApp, 
    _x: usize, 
    _y: usize, 
    px: u32, 
    py: u32, 
    size: u32
) {
    // Determine module color (gradient or solid)
    let color = if app.use_gradient {
        colors::get_gradient_color(px, py, image.width(), image.height(), app)
    } else {
        Rgba([app.fg_color[0], app.fg_color[1], app.fg_color[2], 255])
    };

    // Draw module with selected style
    match app.module_style {
        ModuleStyle::Square => {
            draw_square(image, px, py, size, color);
        }
        ModuleStyle::Circle => {
            draw_circle(image, px, py, size, color);
        }
        ModuleStyle::RoundedSquare => {
            draw_rounded_square(image, px, py, size, color, app);
        }
        ModuleStyle::Dots => {
            draw_dot(image, px, py, size, color);
        }
    }
}

/// Draw an eye module (finder pattern) with the selected style
///
/// Finder patterns are the three 7x7 squares in the QR code corners.
/// Can have different styles and colors from data modules.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `app` - Application state for style settings
/// * `x`, `y` - Module coordinates in QR matrix
/// * `px`, `py` - Pixel coordinates in image
/// * `size` - Size of module in pixels
/// * `eye_positions` - List of eye starting positions to determine which eye
pub fn draw_eye_module(
    image: &mut RgbaImage, 
    app: &QrCodeApp, 
    x: usize, 
    y: usize, 
    px: u32, 
    py: u32,
    size: u32, 
    eye_positions: &[(usize, usize)]
) {
    // Determine eye color (custom, gradient, or foreground)
    let color = if app.use_custom_eye_color {
        Rgba([app.eye_color[0], app.eye_color[1], app.eye_color[2], 255])
    } else if app.use_gradient {
        colors::get_gradient_color(px, py, image.width(), image.height(), app)
    } else {
        Rgba([app.fg_color[0], app.fg_color[1], app.fg_color[2], 255])
    };

    // Find which eye this module belongs to and its relative position
    for (ex, ey) in eye_positions {
        if x >= *ex && x < ex + 7 && y >= *ey && y < ey + 7 {
            let rel_x = x - ex;
            let rel_y = y - ey;
            
            // Draw eye with selected style
            match app.eye_style {
                EyeStyle::Standard => {
                    draw_square(image, px, py, size, color);
                }
                EyeStyle::Circle => {
                    draw_eye_circle(image, px, py, size, color, rel_x, rel_y);
                }
                EyeStyle::RoundedSquare => {
                    draw_eye_rounded(image, px, py, size, color, app);
                }
                EyeStyle::Flower => {
                    draw_eye_flower(image, px, py, size, color, rel_x, rel_y);
                }
                EyeStyle::Diamond => {
                    draw_eye_diamond(image, px, py, size, color, rel_x, rel_y);
                }
            }
            break;
        }
    }
}

// ============================================================================
// Basic Shape Drawing Functions
// ============================================================================

/// Draw a filled square module
///
/// The most basic and reliable module shape.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `x`, `y` - Top-left corner pixel coordinates
/// * `size` - Size in pixels
/// * `color` - RGBA color
pub fn draw_square(image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
    for dy in 0..size {
        for dx in 0..size {
            if x + dx < image.width() && y + dy < image.height() {
                image.put_pixel(x + dx, y + dy, color);
            }
        }
    }
}

/// Draw a filled circle module
///
/// Creates smooth, modern-looking QR codes.
/// Uses distance formula for anti-aliased edges.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `x`, `y` - Top-left corner pixel coordinates
/// * `size` - Bounding box size in pixels
/// * `color` - RGBA color
pub fn draw_circle(image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
    let radius = size as f32 / 2.0;
    let center_x = x as f32 + radius;
    let center_y = y as f32 + radius;

    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;
            
            // Calculate distance from center
            let dist = ((px as f32 - center_x).powi(2) + (py as f32 - center_y).powi(2)).sqrt();
            
            // Only fill pixels within the circle
            if dist <= radius && px < image.width() && py < image.height() {
                image.put_pixel(px, py, color);
            }
        }
    }
}

/// Draw a rounded square module
///
/// Combines the reliability of squares with softer, more appealing aesthetics.
/// Corner radius is adjustable via app settings.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `x`, `y` - Top-left corner pixel coordinates
/// * `size` - Size in pixels
/// * `color` - RGBA color
/// * `app` - Application state for corner radius settings
pub fn draw_rounded_square(
    image: &mut RgbaImage, 
    x: u32, 
    y: u32, 
    size: u32, 
    color: Rgba<u8>,
    app: &QrCodeApp
) {
    // Calculate corner radius
    let radius = if app.use_rounded_corners {
        (size as f32 * app.corner_radius) as u32
    } else {
        (size as f32 * 0.2) as u32 // Default 20% rounding
    };

    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;

            // Determine if we're in a corner region
            let in_corner = (dx < radius && dy < radius) ||                      // Top-left
                           (dx >= size - radius && dy < radius) ||               // Top-right
                           (dx < radius && dy >= size - radius) ||               // Bottom-left
                           (dx >= size - radius && dy >= size - radius);         // Bottom-right

            if in_corner {
                // Calculate distance from nearest corner center
                let corner_x = if dx < radius { radius } else { size - radius };
                let corner_y = if dy < radius { radius } else { size - radius };
                
                let dist = ((dx as f32 - corner_x as f32).powi(2) + 
                           (dy as f32 - corner_y as f32).powi(2)).sqrt();
                
                // Only fill if within corner radius
                if dist <= radius as f32 && px < image.width() && py < image.height() {
                    image.put_pixel(px, py, color);
                }
            } else if px < image.width() && py < image.height() {
                // Not in corner, fill normally
                image.put_pixel(px, py, color);
            }
        }
    }
}

/// Draw a small dot module
///
/// Creates minimalist, artistic QR codes with small circular dots.
/// Dots are 70% of the module size.
///
/// # Arguments
/// * `image` - Image buffer to draw into
/// * `x`, `y` - Top-left corner pixel coordinates
/// * `size` - Bounding box size in pixels
/// * `color` - RGBA color
pub fn draw_dot(image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
    let radius = size as f32 * 0.35; // Dot is 70% of module size
    let center_x = x as f32 + size as f32 / 2.0;
    let center_y = y as f32 + size as f32 / 2.0;

    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;
            
            let dist = ((px as f32 - center_x).powi(2) + (py as f32 - center_y).powi(2)).sqrt();
            
            if dist <= radius && px < image.width() && py < image.height() {
                image.put_pixel(px, py, color);
            }
        }
    }
}

// ============================================================================
// Eye (Finder Pattern) Specific Drawing Functions
// ============================================================================

/// Draw circular eye pattern
///
/// Creates eyes with circular modules, maintaining the finder pattern structure.
///
/// # Arguments
/// * `image` - Image buffer
/// * `px`, `py` - Pixel coordinates
/// * `size` - Module size
/// * `color` - RGBA color
/// * `rel_x`, `rel_y` - Relative position within the 7x7 eye (0-6)
fn draw_eye_circle(
    image: &mut RgbaImage, 
    px: u32, 
    py: u32, 
    size: u32, 
    color: Rgba<u8>, 
    rel_x: usize, 
    rel_y: usize
) {
    // Draw outer ring and center dot as circles
    // Outer ring: edges of the 7x7 square (excluding immediate neighbors of center)
    // Center dot: the single center module
    if (rel_x <= 1 || rel_x >= 5 || rel_y <= 1 || rel_y >= 5) && !(rel_x == 3 && rel_y == 3) {
        draw_circle(image, px, py, size, color);
    } else if rel_x == 3 && rel_y == 3 {
        draw_circle(image, px, py, size, color);
    }
}

/// Draw rounded square eye pattern
///
/// Uses rounded squares for a softer eye appearance.
fn draw_eye_rounded(
    image: &mut RgbaImage, 
    px: u32, 
    py: u32, 
    size: u32, 
    color: Rgba<u8>, 
    app: &QrCodeApp
) {
    draw_rounded_square(image, px, py, size, color, app);
}

/// Draw flower-shaped eye pattern
///
/// Alternates between circles and rounded squares for a decorative petal effect.
///
/// # Arguments
/// * `image` - Image buffer
/// * `px`, `py` - Pixel coordinates
/// * `size` - Module size
/// * `color` - RGBA color
/// * `rel_x`, `rel_y` - Relative position within eye
fn draw_eye_flower(
    image: &mut RgbaImage, 
    px: u32, 
    py: u32, 
    size: u32, 
    color: Rgba<u8>, 
    rel_x: usize, 
    rel_y: usize
) {
    let is_outer = rel_x == 0 || rel_x == 6 || rel_y == 0 || rel_y == 6;
    let is_inner = (rel_x >= 2 && rel_x <= 4) && (rel_y >= 2 && rel_y <= 4);
    
    if is_outer || is_inner {
        // Alternate pattern based on position
        if (rel_x + rel_y) % 2 == 0 {
            draw_circle(image, px, py, size, color);
        } else {
            // Use default rounding for flower petals
            let default_app = crate::app::QrCodeApp::default();
            draw_rounded_square(image, px, py, size, color, &default_app);
        }
    }
}

/// Draw diamond-shaped eye pattern
///
/// Creates a geometric diamond shape using Manhattan distance.
///
/// # Arguments
/// * `image` - Image buffer
/// * `px`, `py` - Pixel coordinates
/// * `size` - Module size
/// * `color` - RGBA color
/// * `rel_x`, `rel_y` - Relative position within eye
fn draw_eye_diamond(
    image: &mut RgbaImage, 
    px: u32, 
    py: u32, 
    size: u32, 
    color: Rgba<u8>, 
    rel_x: usize, 
    rel_y: usize
) {
    let cx = 3; // Center x
    let cy = 3; // Center y
    
    // Calculate Manhattan distance from center
    let dist = ((rel_x as i32 - cx).abs() + (rel_y as i32 - cy).abs()) as usize;
    
    // Draw only modules at specific distances to form diamond
    if dist == 3 || dist == 1 {
        draw_square(image, px, py, size, color);
    }
}
