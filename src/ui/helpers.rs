//! UI helper functions
//!
//! Provides reusable UI widgets and utilities.

use eframe::egui;

/// Render an RGB color picker widget
///
/// Creates an interactive color picker that modifies the provided color array.
/// Color values are stored as u8 (0-255) and converted to/from f32 for egui.
///
/// # Arguments
/// * `ui` - egui UI context to render into
/// * `color` - Mutable reference to RGB color array [R, G, B]
///
/// # Example
/// ```
/// let mut my_color = [255, 128, 0]; // Orange
/// color_picker(ui, &mut my_color);
/// // User can now interact with color picker
/// ```
pub fn color_picker(ui: &mut egui::Ui, color: &mut [u8; 3]) {
    // Convert u8 (0-255) to f32 (0.0-1.0) for egui
    let mut color_f32 = [
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
    ];

    // Show color picker button
    if ui.color_edit_button_rgb(&mut color_f32).changed() {
        // Convert f32 back to u8 when changed
        color[0] = (color_f32[0] * 255.0) as u8;
        color[1] = (color_f32[1] * 255.0) as u8;
        color[2] = (color_f32[2] * 255.0) as u8;
    }
}
