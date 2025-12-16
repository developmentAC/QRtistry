//! Core QR code generation logic
//!
//! Handles the creation of QR code images from text input,
//! applying all styling options like colors, gradients, module styles,
//! eye patterns, logos, and background images.

use image::{ImageBuffer, Rgba, RgbaImage, imageops};
use qrcode::QrCode;

use crate::app::QrCodeApp;
use crate::qr::{drawing, images};

/// Generate a fully styled QR code image based on application settings
///
/// This is the main entry point for QR code creation. It:
/// 1. Generates the base QR code matrix
/// 2. Creates an appropriately sized image buffer
/// 3. Applies background image (if present)
/// 4. Draws all QR modules with the selected style
/// 5. Applies logo overlay (if present)
/// 6. Applies opacity settings
///
/// # Arguments
/// * `app` - Application state containing all QR code settings
///
/// # Returns
/// * `Ok(RgbaImage)` - Successfully generated QR code image
/// * `Err(String)` - Error message describing what went wrong
///
/// # Errors
/// Returns an error if:
/// - QR code content is invalid or too long
/// - Image operations fail
/// - Logo overlay fails
pub fn generate_qr_image(app: &QrCodeApp) -> Result<RgbaImage, String> {
    // === Step 1: Generate QR Code Matrix ===
    let code = QrCode::with_error_correction_level(
        &app.qr_text, 
        app.ec_level.to_ec_level()
    ).map_err(|e| format!("Failed to create QR code: {}", e))?;

    let matrix = code.to_colors();
    let qr_width = code.width();

    // === Step 2: Calculate Dimensions ===
    // Module size in pixels (how big each black/white square is)
    let module_size = (app.size - 2 * app.border * (app.size / qr_width as u32)) / qr_width as u32;
    let actual_qr_size = module_size * qr_width as u32;
    let total_size = actual_qr_size + 2 * app.border * module_size;

    // === Step 3: Create Base Image ===
    let mut image: RgbaImage = if let Some(bg_img) = &app.bg_image {
        // Use background image if provided
        create_background_with_image(bg_img, total_size, app)
    } else {
        // Otherwise, solid color background
        create_solid_background(total_size, app.bg_color)
    };

    // === Step 4: Identify Eye (Finder Pattern) Positions ===
    // Eyes are the three 7x7 squares in the corners
    let eye_positions = vec![
        (0, 0),                    // Top-left
        (qr_width - 7, 0),         // Top-right
        (0, qr_width - 7),         // Bottom-left
    ];

    // === Step 5: Draw All QR Modules ===
    let offset = app.border * module_size;
    
    for y in 0..qr_width {
        for x in 0..qr_width {
            // Only draw dark modules (white modules are already background)
            let is_dark = matches!(matrix[y * qr_width + x], qrcode::Color::Dark);
            
            if is_dark {
                let px = offset + x as u32 * module_size;
                let py = offset + y as u32 * module_size;
                
                // Check if this module is part of a finder pattern
                let is_eye = eye_positions.iter().any(|(ex, ey)| {
                    x >= *ex && x < ex + 7 && y >= *ey && y < ey + 7
                });
                
                if is_eye {
                    // Use eye-specific drawing
                    drawing::draw_eye_module(
                        &mut image, app, x, y, px, py, 
                        module_size, &eye_positions
                    );
                } else {
                    // Use data module drawing
                    drawing::draw_data_module(
                        &mut image, app, x, y, px, py, 
                        module_size
                    );
                }
            }
        }
    }

    // === Step 6: Apply Logo Overlay ===
    if let Some(logo_img) = &app.logo_image {
        images::apply_logo_overlay(
            &mut image, logo_img, qr_width, 
            module_size, offset, app.logo_size
        )?;
    }

    // === Step 7: Apply Overall Opacity ===
    if app.qr_opacity < 1.0 {
        apply_qr_opacity(&mut image, app.qr_opacity, app.bg_color);
    }

    Ok(image)
}

/// Create a solid color background image
///
/// # Arguments
/// * `size` - Image dimensions (square)
/// * `bg_color` - RGB color for background
///
/// # Returns
/// RGBA image filled with the specified color
fn create_solid_background(size: u32, bg_color: [u8; 3]) -> RgbaImage {
    let mut img = ImageBuffer::new(size, size);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([bg_color[0], bg_color[1], bg_color[2], 255]);
    }
    img
}

/// Create background by blending a background image with solid color
///
/// # Arguments
/// * `bg_img` - Background image to use
/// * `size` - Target size for the background
/// * `app` - Application state for opacity and background color
///
/// # Returns
/// RGBA image with blended background
fn create_background_with_image(
    bg_img: &image::DynamicImage, 
    size: u32, 
    app: &QrCodeApp
) -> RgbaImage {
    // Resize background image to match QR code size
    let resized = bg_img.resize_exact(size, size, imageops::FilterType::Lanczos3);
    let mut img_buffer = resized.to_rgba8();
    
    // Apply opacity to background image
    if app.bg_image_opacity < 1.0 {
        for pixel in img_buffer.pixels_mut() {
            pixel[3] = (pixel[3] as f32 * app.bg_image_opacity) as u8;
        }
    }
    
    // Create solid color base
    let mut base = ImageBuffer::new(size, size);
    for pixel in base.pixels_mut() {
        *pixel = Rgba([app.bg_color[0], app.bg_color[1], app.bg_color[2], 255]);
    }
    
    // Blend background image over solid color
    imageops::overlay(&mut base, &img_buffer, 0, 0);
    base
}

/// Apply opacity to QR code modules (not background)
///
/// Useful for watermark effects or subtle integration with backgrounds.
///
/// # Arguments
/// * `image` - Image to modify
/// * `opacity` - Opacity value (0.0 = transparent, 1.0 = opaque)
/// * `bg_color` - Background color to distinguish from QR modules
fn apply_qr_opacity(image: &mut RgbaImage, opacity: f32, bg_color: [u8; 3]) {
    for pixel in image.pixels_mut() {
        // Only apply opacity to non-background pixels
        if pixel[0] != bg_color[0] || pixel[1] != bg_color[1] || pixel[2] != bg_color[2] {
            pixel[3] = (pixel[3] as f32 * opacity) as u8;
        }
    }
}
