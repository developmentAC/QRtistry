//! Main application state and configuration
//!
//! This module defines the core QrCodeApp struct that holds all application state,
//! including user settings, runtime data, and UI state. It also implements the
//! eframe::App trait for the main GUI update loop.

use eframe::egui;
use image::DynamicImage;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

use crate::types::*;
use crate::qr;
use crate::ui;
use crate::io;

/// Main application state structure
///
/// Contains all configuration options, runtime state, and UI data.
/// Most fields are serializable for preset save/load functionality.
#[derive(Serialize, Deserialize)]
pub struct QrCodeApp {
    // === Content Settings ===
    /// Text content to encode in the QR code
    pub qr_text: String,
    
    /// Output size of the QR code image in pixels (128-2048)
    pub size: u32,
    
    /// Border width in modules (quiet zone around QR code)
    pub border: u32,
    
    /// Error correction level (affects reliability and capacity)
    pub ec_level: ErrorCorrectionLevel,
    
    // === Color Settings ===
    /// Foreground color for dark modules (RGB 0-255)
    pub fg_color: [u8; 3],
    
    /// Background color for light areas (RGB 0-255)
    pub bg_color: [u8; 3],
    
    /// Enable gradient color blending
    pub use_gradient: bool,
    
    /// Type of gradient to apply (horizontal, vertical, etc.)
    pub gradient_type: GradientType,
    
    /// Second color for gradient blending (RGB 0-255)
    pub gradient_color: [u8; 3],
    
    // === Module Styling ===
    /// Visual style for data modules (square, circle, etc.)
    pub module_style: ModuleStyle,
    
    /// Enable extra rounding on rounded modules
    pub use_rounded_corners: bool,
    
    /// Corner radius for rounded modules (0.0-1.0)
    pub corner_radius: f32,
    
    // === Eye (Finder Pattern) Styling ===
    /// Visual style for the three corner finder patterns
    pub eye_style: EyeStyle,
    
    /// Use a custom color for eye patterns
    pub use_custom_eye_color: bool,
    
    /// Custom color for eye patterns if enabled (RGB 0-255)
    pub eye_color: [u8; 3],
    
    // === Image Integration ===
    /// Path to logo image file (not serialized)
    #[serde(skip)]
    pub logo_path: Option<PathBuf>,
    
    /// Loaded logo image data (not serialized)
    #[serde(skip)]
    pub logo_image: Option<DynamicImage>,
    
    /// Logo size as fraction of QR code (0.05-0.35)
    pub logo_size: f32,
    
    /// Path to background image file (not serialized)
    #[serde(skip)]
    pub bg_image_path: Option<PathBuf>,
    
    /// Loaded background image data (not serialized)
    #[serde(skip)]
    pub bg_image: Option<DynamicImage>,
    
    /// Background image opacity (0.0-1.0)
    pub bg_image_opacity: f32,
    
    // === Advanced Settings ===
    /// Overall QR code opacity (0.0-1.0) for watermark effects
    pub qr_opacity: f32,
    
    // === UI State ===
    /// Currently selected tab in the UI
    pub selected_tab: TabSelection,
    
    /// Cached preview texture for display (not serialized)
    #[serde(skip)]
    pub preview_texture: Option<egui::TextureHandle>,
    
    /// Status message displayed to user (not serialized)
    #[serde(skip)]
    pub status_message: String,
    
    /// Flag to trigger auto-preview on first frame (not serialized)
    #[serde(skip)]
    pub first_frame: bool,
}

impl Default for QrCodeApp {
    /// Create a new application with sensible default settings
    fn default() -> Self {
        Self {
            // Default content
            qr_text: String::from("https://oliverbonhamcarter.com"),
            
            // Default dimensions
            size: 512,
            border: 2,
            ec_level: ErrorCorrectionLevel::Medium,
            
            // Default colors (classic black on white)
            fg_color: [0, 0, 0],
            bg_color: [255, 255, 255],
            use_gradient: false,
            gradient_type: GradientType::Horizontal,
            gradient_color: [100, 100, 255],
            
            // Default module style (classic square)
            module_style: ModuleStyle::Square,
            use_rounded_corners: false,
            corner_radius: 0.3,
            
            // Default eye style (standard)
            eye_style: EyeStyle::Standard,
            use_custom_eye_color: false,
            eye_color: [255, 0, 0],
            
            // No images by default
            logo_path: None,
            logo_image: None,
            logo_size: 0.2,
            bg_image_path: None,
            bg_image: None,
            bg_image_opacity: 0.3,
            
            // Default opacity (fully opaque)
            qr_opacity: 1.0,
            
            // UI state
            selected_tab: TabSelection::Basic,
            preview_texture: None,
            status_message: String::from("Ready to generate QR code"),
            first_frame: true,
        }
    }
}

impl eframe::App for QrCodeApp {
    /// Main GUI update loop called every frame
    ///
    /// Handles the panel-based layout and delegates rendering to specialized functions.
    ///
    /// # Arguments
    /// * `ctx` - egui context for rendering UI elements
    /// * `_frame` - Frame handle (unused)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-generate preview on first frame for immediate visual feedback
        if self.first_frame {
            self.first_frame = false;
            self.generate_preview(ctx);
        }
        
        // === Top Panel: Title and Action Buttons ===
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎨 QRtistry");
                ui.add_space(20.0);
                
                // Right-aligned action buttons
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔃 Reset").clicked() {
                        *self = Self::default();
                        self.first_frame = true;
                    }
                    
                    if ui.button("📂 Load Preset").clicked() {
                        io::load_preset(self, ctx);
                    }
                    
                    if ui.button("📋 Save Preset").clicked() {
                        io::save_preset(self);
                    }
                    
                    if ui.button("💾 Save PNG").clicked() {
                        io::save_qr_code(self);
                    }
                    
                    if ui.button("🔄 Generate Preview").clicked() {
                        self.generate_preview(ctx);
                    }
                });
            });
        });
        
        // === Bottom Panel: Status Bar ===
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
            });
        });
        
        // === Left Panel: Resizable Control Panel ===
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(400.0)
            .width_range(350.0..=600.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui::render_controls(self, ui, ctx);
                    });
            });
        
        // === Central Panel: Large Preview Area ===
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::render_preview(self, ui, ctx);
        });
    }
}

impl QrCodeApp {
    /// Generate QR code and update preview texture
    ///
    /// This is the main entry point for creating the QR code visual.
    /// It generates the image using the QR module and converts it to an egui texture.
    ///
    /// # Arguments
    /// * `ctx` - egui context for texture loading
    pub fn generate_preview(&mut self, ctx: &egui::Context) {
        // Validate input
        if self.qr_text.is_empty() {
            self.status_message = "⚠️ Please enter text for the QR code".to_string();
            return;
        }

        self.status_message = "🔄 Generating QR code...".to_string();
        
        // Generate QR code image
        match qr::generate_qr_image(self) {
            Ok(img) => {
                let width = img.width() as usize;
                let height = img.height() as usize;
                
                // Convert RGBA image to egui Color32 format
                let pixels: Vec<egui::Color32> = img.pixels()
                    .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                    .collect();
                
                let color_image = egui::ColorImage {
                    size: [width, height],
                    pixels,
                };
                
                // Load texture into GPU memory
                self.preview_texture = Some(ctx.load_texture(
                    "qr_preview",
                    color_image,
                    egui::TextureOptions::NEAREST, // Nearest neighbor for sharp pixels
                ));
                
                self.status_message = format!("✅ QR code generated successfully! ({}x{})", width, height);
            }
            Err(e) => {
                self.status_message = format!("❌ Error: {}", e);
            }
        }
    }
}
