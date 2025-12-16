//! File input/output operations
//!
//! Handles saving and loading:
//! - QR code PNG export
//! - Configuration presets (JSON)

use chrono::Local;
use image;

use crate::app::QrCodeApp;
use crate::qr;

/// Save QR code as PNG file with file dialog
///
/// Opens a native file save dialog and exports the current QR code design
/// as a PNG image file. Uses timestamp-based filename by default.
///
/// # Arguments
/// * `app` - Application state containing QR code settings
pub fn save_qr_code(app: &mut QrCodeApp) {
    // Validate input
    if app.qr_text.is_empty() {
        app.status_message = "⚠️ Please enter text for the QR code".to_string();
        return;
    }

    // Generate default filename with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let default_filename = format!("qrcode_{}.png", timestamp);

    // Open file save dialog
    let file = rfd::FileDialog::new()
        .set_file_name(&default_filename)
        .add_filter("PNG Image", &["png"])
        .save_file();

    if let Some(path) = file {
        // Generate QR code image
        match qr::generate_qr_image(app) {
            Ok(image) => {
                // Save to file
                match image.save(&path) {
                    Ok(_) => {
                        app.status_message = format!("✅ Saved to: {}", path.display());
                    }
                    Err(e) => {
                        app.status_message = format!("❌ Failed to save: {}", e);
                    }
                }
            }
            Err(e) => {
                app.status_message = format!("❌ Error generating QR code: {}", e);
            }
        }
    } else {
        app.status_message = "Save cancelled".to_string();
    }
}

/// Save current configuration as JSON preset
///
/// Opens a file save dialog and exports all serializable application settings
/// as a JSON file for later reuse.
///
/// **Note**: Image file paths are not saved, only settings.
///
/// # Arguments
/// * `app` - Application state to serialize
pub fn save_preset(app: &mut QrCodeApp) {
    // Generate default filename with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let default_filename = format!("qr_preset_{}.json", timestamp);

    // Open file save dialog
    let file = rfd::FileDialog::new()
        .set_file_name(&default_filename)
        .add_filter("JSON Preset", &["json"])
        .save_file();

    if let Some(path) = file {
        // Serialize application state to pretty JSON
        match serde_json::to_string_pretty(app) {
            Ok(json) => {
                // Write JSON to file
                match std::fs::write(&path, json) {
                    Ok(_) => {
                        app.status_message = format!("✅ Preset saved to: {}", path.display());
                    }
                    Err(e) => {
                        app.status_message = format!("❌ Failed to save preset: {}", e);
                    }
                }
            }
            Err(e) => {
                app.status_message = format!("❌ Failed to serialize preset: {}", e);
            }
        }
    } else {
        app.status_message = "Save cancelled".to_string();
    }
}

/// Load configuration preset from JSON file
///
/// Opens a file open dialog and loads a previously saved configuration preset.
/// Automatically generates preview after loading.
///
/// **Note**: Image file paths in the preset are attempted to be reloaded,
/// but may fail if files have moved.
///
/// # Arguments
/// * `app` - Application state to update with loaded preset
/// * `ctx` - egui context for triggering preview generation
pub fn load_preset(app: &mut QrCodeApp, ctx: &eframe::egui::Context) {
    // Open file open dialog
    let file = rfd::FileDialog::new()
        .add_filter("JSON Preset", &["json"])
        .pick_file();

    if let Some(path) = file {
        // Read JSON file
        match std::fs::read_to_string(&path) {
            Ok(json) => {
                // Deserialize JSON to application state
                match serde_json::from_str::<QrCodeApp>(&json) {
                    Ok(mut loaded) => {
                        // Preserve runtime-only fields that shouldn't be overwritten
                        loaded.preview_texture = None;
                        loaded.status_message = format!("✅ Preset loaded from: {}", path.display());
                        loaded.selected_tab = app.selected_tab; // Keep current tab
                        
                        // Attempt to reload image files if paths exist
                        // (May fail if files have been moved/deleted)
                        if let Some(logo_path) = &loaded.logo_path {
                            if let Ok(img) = image::open(logo_path) {
                                loaded.logo_image = Some(img);
                            } else {
                                loaded.logo_path = None; // Clear if file not found
                            }
                        }
                        
                        if let Some(bg_path) = &loaded.bg_image_path {
                            if let Ok(img) = image::open(bg_path) {
                                loaded.bg_image = Some(img);
                            } else {
                                loaded.bg_image_path = None; // Clear if file not found
                            }
                        }
                        
                        // Update application state
                        *app = loaded;
                        
                        // Auto-generate preview with new settings
                        app.generate_preview(ctx);
                    }
                    Err(e) => {
                        app.status_message = format!("❌ Failed to parse preset: {}", e);
                    }
                }
            }
            Err(e) => {
                app.status_message = format!("❌ Failed to read preset file: {}", e);
            }
        }
    } else {
        app.status_message = "Load cancelled".to_string();
    }
}
