//! Image integration: logos and background images
//!
//! Handles overlaying logos onto QR codes and blending background images.

use image::{DynamicImage, imageops, RgbaImage};

/// Apply a logo overlay to the center of the QR code
///
/// The logo is resized to the specified size and centered on the QR code.
/// Uses alpha blending to preserve logo transparency.
///
/// **Important**: Logos reduce scannability! Use high error correction
/// and keep logo size under 30% for best results.
///
/// # Arguments
/// * `image` - QR code image to overlay logo onto
/// * `logo` - Logo image to overlay
/// * `qr_width` - Width of QR matrix in modules
/// * `module_size` - Size of each module in pixels
/// * `offset` - Border offset in pixels
/// * `logo_size_ratio` - Logo size as fraction of QR code (0.05-0.35)
///
/// # Returns
/// * `Ok(())` - Logo successfully applied
/// * `Err(String)` - Error message if overlay fails
///
/// # Example
/// ```
/// // Add a logo that's 20% of the QR code size
/// apply_logo_overlay(&mut qr_image, &logo, qr_width, module_size, offset, 0.2)?;
/// ```
pub fn apply_logo_overlay(
    image: &mut RgbaImage, 
    logo: &DynamicImage, 
    qr_width: usize, 
    module_size: u32, 
    offset: u32,
    logo_size_ratio: f32
) -> Result<(), String> {
    // === Step 1: Calculate Logo Dimensions ===
    let qr_size = qr_width as u32 * module_size;
    let logo_size = (qr_size as f32 * logo_size_ratio) as u32;
    
    // Validate logo size
    if logo_size == 0 {
        return Err("Logo size too small to render".to_string());
    }
    if logo_size > qr_size {
        return Err("Logo size exceeds QR code dimensions".to_string());
    }
    
    // === Step 2: Resize Logo ===
    // Use high-quality Lanczos filter for best appearance
    let resized_logo = logo.resize_exact(
        logo_size, 
        logo_size, 
        imageops::FilterType::Lanczos3
    );
    let logo_rgba = resized_logo.to_rgba8();
    
    // === Step 3: Calculate Center Position ===
    // Logo is centered within the QR code area (excluding border)
    let center_x = offset + (qr_size - logo_size) / 2;
    let center_y = offset + (qr_size - logo_size) / 2;
    
    // === Step 4: Overlay with Alpha Blending ===
    // This preserves logo transparency and blends nicely with QR modules
    imageops::overlay(
        image, 
        &logo_rgba, 
        center_x as i64, 
        center_y as i64
    );
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logo_size_calculation() {
        let qr_width = 25;
        let module_size = 10;
        let logo_ratio = 0.2;
        
        let qr_size = qr_width as u32 * module_size;
        let logo_size = (qr_size as f32 * logo_ratio) as u32;
        
        assert_eq!(logo_size, 50); // 250 * 0.2 = 50
    }

    #[test]
    fn test_logo_center_calculation() {
        let offset = 20;
        let qr_size = 250;
        let logo_size = 50;
        
        let center_x = offset + (qr_size - logo_size) / 2;
        let center_y = offset + (qr_size - logo_size) / 2;
        
        assert_eq!(center_x, 120); // 20 + (250-50)/2 = 20 + 100 = 120
        assert_eq!(center_y, 120);
    }
}
