//! Preview panel rendering
//!
//! Displays the generated QR code in the central panel with appropriate sizing.

use eframe::egui;

use crate::app::QrCodeApp;

/// Render the QR code preview panel
///
/// Displays the QR code texture in the center of the UI with white background.
/// Automatically scales to use available space while maintaining aspect ratio.
///
/// # Arguments
/// * `app` - Application state containing preview texture
/// * `ui` - egui UI context to render into
/// * `ctx` - egui context for triggering preview generation
pub fn render_preview(app: &mut QrCodeApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.heading("Preview");
    ui.separator();

    if let Some(texture) = &app.preview_texture {
        // Get all available space in the central panel
        let available = ui.available_size();
        
        // Use 90% of available space, minimum 300px, maximum 800px
        let size = (available.x.min(available.y) * 0.9).max(300.0).min(800.0);
        
        // Center the preview
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            
            // Draw white background for QR code
            let rect_pos = ui.cursor().min;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(rect_pos, egui::vec2(size, size)),
                0.0, // No rounding
                egui::Color32::WHITE,
            );
            
            // Display QR code texture
            ui.image((texture.id(), egui::vec2(size, size)));
            
            ui.add_space(10.0);
            
            // Show dimensions
            ui.label(format!(
                "📐 {} x {} pixels", 
                texture.size()[0], 
                texture.size()[1]
            ));
        });
    } else {
        // No preview available yet
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("⏳ Loading...");
            ui.add_space(20.0);
            ui.label("Your QR code will appear here");
            ui.add_space(20.0);
            
            // Manual generation button
            if ui.button("🔄 Generate Now").clicked() {
                app.generate_preview(ctx);
            }
        });
    }
}
