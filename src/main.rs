//! QRtistry
//!
//! A professional, feature-rich GUI application for creating beautifully
//! customized QR codes with real-time preview and advanced styling options.
//!
//! # Features
//! - Interactive text input for QR code content
//! - Custom colors with gradients (horizontal, vertical, diagonal, radial)
//! - Adjustable dimensions and borders
//! - Multiple error correction levels
//! - 8 color presets for quick styling
//! - Module shape customization (square, circle, rounded, dots)
//! - Custom eye (finder pattern) styles and colors
//! - Logo/image overlay support with size control
//! - Background image blending with opacity
//! - Transparency control for watermark effects
//! - Real-time preview with large display area
//! - Save/load preset configurations as JSON
//! - Export to PNG with timestamp-based filenames
//! - Resizable panel-based UI layout
//!
//! # Architecture
//! The application is organized into logical modules:
//! - `app`: Main application state and GUI update loop
//! - `types`: Enums and data structures
//! - `qr`: QR code generation, drawing, colors, and image operations
//! - `ui`: User interface rendering (tabs, preview, helpers)
//! - `io`: File input/output operations
//!
//! # Usage
//! ```bash
//! cargo run --release
//! ```
//!
//! The application window opens at 1200×800px with:
//! - Top bar: Title and action buttons
//! - Left panel: Resizable settings (350-600px width)
//! - Center panel: Large QR code preview
//! - Bottom bar: Status messages

use eframe;

// Module declarations
mod app;
mod types;
mod qr;
mod ui;
mod io;

/// Application entry point
///
/// Initializes the eframe window and starts the GUI event loop.
///
/// # Returns
/// * `Ok(())` - Application exited normally
/// * `Err(eframe::Error)` - Application failed to start
fn main() -> eframe::Result<()> {
    // Configure window options
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])        // Default window size
            .with_min_inner_size([1000.0, 700.0]),   // Minimum window size
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "QRtistry",                                  // Window title
        options,
        Box::new(|_cc| Ok(Box::new(app::QrCodeApp::default()))),
    )
}
