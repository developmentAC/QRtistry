//! QR Code Generator with Interactive GUI
//! 
//! This application provides a rich graphical interface for creating customized QR codes
//! with various styling options, colors, borders, and creative features.
//! 
//! # Features
//! - Interactive text input for QR code content
//! - Custom foreground and background colors with gradients
//! - Adjustable dimensions and borders
//! - Multiple error correction levels
//! - Color presets for quick styling
//! - Module shape customization
//! - Custom eye (finder pattern) styles and colors
//! - Logo/image overlay support
//! - Background image blending
//! - Transparency control
//! - Real-time preview
//! - Save/load preset configurations
//! - Save to PNG with file dialog

use eframe::egui;
use image::{ImageBuffer, Rgba, RgbaImage, DynamicImage, imageops};
use qrcode::{QrCode, EcLevel};
use std::path::PathBuf;
use chrono::Local;
use serde::{Serialize, Deserialize};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "QR Code Designer Studio Pro",
        options,
        Box::new(|_cc| Ok(Box::new(QrCodeApp::default()))),
    )
}

/// Main application state and configuration
#[derive(Serialize, Deserialize)]
struct QrCodeApp {
    /// Text content to encode in the QR code
    qr_text: String,
    
    /// Foreground color (dark modules) as RGB
    fg_color: [u8; 3],
    
    /// Background color (light modules) as RGB
    bg_color: [u8; 3],
    
    /// Size of the QR code in pixels
    size: u32,
    
    /// Border width in modules
    border: u32,
    
    /// Error correction level
    ec_level: ErrorCorrectionLevel,
    
    /// Module shape style
    module_style: ModuleStyle,
    
    /// Preview texture for displaying QR code
    #[serde(skip)]
    preview_texture: Option<egui::TextureHandle>,
    
    /// Status message to display to user
    #[serde(skip)]
    status_message: String,
    
    /// Whether to use rounded corners for modules
    use_rounded_corners: bool,
    
    /// Corner radius for rounded modules (0.0 to 1.0)
    corner_radius: f32,
    
    /// Enable gradient colors
    use_gradient: bool,
    
    /// Gradient type
    gradient_type: GradientType,
    
    /// Second color for gradient
    gradient_color: [u8; 3],
    
    /// Custom eye style
    eye_style: EyeStyle,
    
    /// Custom eye color (if enabled)
    use_custom_eye_color: bool,
    eye_color: [u8; 3],
    
    /// Logo overlay path
    #[serde(skip)]
    logo_path: Option<PathBuf>,
    #[serde(skip)]
    logo_image: Option<DynamicImage>,
    
    /// Logo size percentage (0.0 to 0.3)
    logo_size: f32,
    
    /// Background image path
    #[serde(skip)]
    bg_image_path: Option<PathBuf>,
    #[serde(skip)]
    bg_image: Option<DynamicImage>,
    
    /// Background image opacity (0.0 to 1.0)
    bg_image_opacity: f32,
    
    /// Overall QR code opacity (0.0 to 1.0)
    qr_opacity: f32,
    
    /// Tab selection
    #[serde(skip)]
    selected_tab: TabSelection,
    
    /// Track if this is first frame (for auto-preview)
    #[serde(skip)]
    first_frame: bool,
}

/// Tab selection for organized UI
#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TabSelection {
    #[default]
    Basic,
    Style,
    Advanced,
    Images,
}

/// Gradient type options
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum GradientType {
    /// Horizontal gradient (left to right)
    Horizontal,
    /// Vertical gradient (top to bottom)
    Vertical,
    /// Radial gradient (center outward)
    Radial,
    /// Diagonal gradient
    Diagonal,
}

/// Eye (finder pattern) style options
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum EyeStyle {
    /// Standard square eyes
    Standard,
    /// Circular eyes
    Circle,
    /// Rounded square eyes
    RoundedSquare,
    /// Flower-like pattern
    Flower,
    /// Diamond shape
    Diamond,
}

/// Error correction level options for QR codes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum ErrorCorrectionLevel {
    /// Low: ~7% error correction
    Low,
    /// Medium: ~15% error correction  
    Medium,
    /// Quartile: ~25% error correction
    Quartile,
    /// High: ~30% error correction
    High,
}

impl ErrorCorrectionLevel {
    /// Convert to qrcode crate's EcLevel
    fn to_ec_level(&self) -> EcLevel {
        match self {
            ErrorCorrectionLevel::Low => EcLevel::L,
            ErrorCorrectionLevel::Medium => EcLevel::M,
            ErrorCorrectionLevel::Quartile => EcLevel::Q,
            ErrorCorrectionLevel::High => EcLevel::H,
        }
    }
}

/// Different visual styles for QR code modules
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum ModuleStyle {
    /// Standard square modules
    Square,
    /// Circular modules
    Circle,
    /// Rounded square modules
    RoundedSquare,
    /// Dot-style modules
    Dots,
}

/// Predefined color schemes for quick styling
struct ColorPreset {
    name: &'static str,
    fg: [u8; 3],
    bg: [u8; 3],
}

const COLOR_PRESETS: &[ColorPreset] = &[
    ColorPreset { name: "Classic", fg: [0, 0, 0], bg: [255, 255, 255] },
    ColorPreset { name: "Ocean", fg: [0, 119, 182], bg: [224, 247, 250] },
    ColorPreset { name: "Sunset", fg: [255, 87, 34], bg: [255, 243, 224] },
    ColorPreset { name: "Forest", fg: [27, 94, 32], bg: [232, 245, 233] },
    ColorPreset { name: "Purple", fg: [123, 31, 162], bg: [243, 229, 245] },
    ColorPreset { name: "Rose", fg: [194, 24, 91], bg: [252, 228, 236] },
    ColorPreset { name: "Night", fg: [255, 255, 255], bg: [33, 33, 33] },
    ColorPreset { name: "Cyber", fg: [0, 255, 255], bg: [10, 10, 40] },
];

impl Default for QrCodeApp {
    fn default() -> Self {
        Self {
            qr_text: String::from("https://oliverbonhamcarter.com"),
            fg_color: [0, 0, 0],
            bg_color: [255, 255, 255],
            size: 512,
            border: 2,
            ec_level: ErrorCorrectionLevel::Medium,
            module_style: ModuleStyle::Square,
            preview_texture: None,
            status_message: String::from("Ready to generate QR code"),
            use_rounded_corners: false,
            corner_radius: 0.3,
            use_gradient: false,
            gradient_type: GradientType::Horizontal,
            gradient_color: [100, 100, 255],
            eye_style: EyeStyle::Standard,
            use_custom_eye_color: false,
            eye_color: [255, 0, 0],
            logo_path: None,
            logo_image: None,
            logo_size: 0.2,
            bg_image_path: None,
            bg_image: None,
            bg_image_opacity: 0.3,
            qr_opacity: 1.0,
            selected_tab: TabSelection::Basic,
            first_frame: true,
        }
    }
}

impl eframe::App for QrCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-generate preview on first frame
        if self.first_frame {
            self.first_frame = false;
            self.generate_preview(ctx);
        }
        
        // Top panel for title and action buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎨 QR Code Designer Studio Pro");
                ui.add_space(20.0);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔃 Reset").clicked() {
                        *self = Self::default();
                        self.first_frame = true;
                    }
                    
                    if ui.button("📂 Load Preset").clicked() {
                        self.load_preset(ctx);
                    }
                    
                    if ui.button("📋 Save Preset").clicked() {
                        self.save_preset();
                    }
                    
                    if ui.button("💾 Save PNG").clicked() {
                        self.save_qr_code();
                    }
                    
                    if ui.button("🔄 Generate Preview").clicked() {
                        self.generate_preview(ctx);
                    }
                });
            });
        });
        
        // Bottom panel for status
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
            });
        });
        
        // Left panel for controls
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(400.0)
            .width_range(350.0..=600.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.render_controls(ui, ctx);
                    });
            });
        
        // Central panel for preview
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_preview(ui, ctx);
        });
    }
}

impl QrCodeApp {
    /// Render the control panel UI with tabs
    fn render_controls(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.heading("Settings");
        ui.add_space(10.0);

        // Tab selection
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.selected_tab, TabSelection::Basic, "📝 Basic");
            ui.selectable_value(&mut self.selected_tab, TabSelection::Style, "🎨 Style");
            ui.selectable_value(&mut self.selected_tab, TabSelection::Advanced, "⚙️ Advanced");
            ui.selectable_value(&mut self.selected_tab, TabSelection::Images, "🖼️ Images");
        });

        ui.separator();
        ui.add_space(10.0);

        match self.selected_tab {
            TabSelection::Basic => self.render_basic_tab(ui),
            TabSelection::Style => self.render_style_tab(ui),
            TabSelection::Advanced => self.render_advanced_tab(ui),
            TabSelection::Images => self.render_images_tab(ui),
        }
    }

    /// Render basic settings tab
    fn render_basic_tab(&mut self, ui: &mut egui::Ui) {
        // Text Input Section
        ui.group(|ui| {
            ui.label("📝 QR Code Content:");
            ui.add(
                egui::TextEdit::multiline(&mut self.qr_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(8)
            );
            ui.label(format!("Characters: {}", self.qr_text.len()));
        });

        ui.add_space(10.0);

        // Dimensions Section
        ui.group(|ui| {
            ui.label("📐 Dimensions:");
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Size:");
                ui.add(egui::Slider::new(&mut self.size, 128..=2048).suffix(" px"));
            });
            
            ui.add_space(8.0);
            
            ui.horizontal(|ui| {
                ui.label("Border:");
                ui.add(egui::Slider::new(&mut self.border, 0..=10).suffix(" modules"));
            });
            
            ui.add_space(5.0);
        });

        ui.add_space(10.0);

        // Error Correction Section
        ui.group(|ui| {
            ui.label("🛡️ Error Correction:");
            ui.add_space(5.0);
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.ec_level, ErrorCorrectionLevel::Low, "Low");
                ui.selectable_value(&mut self.ec_level, ErrorCorrectionLevel::Medium, "Medium");
                ui.selectable_value(&mut self.ec_level, ErrorCorrectionLevel::Quartile, "Quartile");
                ui.selectable_value(&mut self.ec_level, ErrorCorrectionLevel::High, "High");
            });
            ui.add_space(5.0);
            ui.label("Higher levels allow more decoration while maintaining scannability");
        });
    }

    /// Render style settings tab
    fn render_style_tab(&mut self, ui: &mut egui::Ui) {
        // Color Section
        ui.group(|ui| {
            ui.label("🎨 Colors:");
            
            ui.horizontal(|ui| {
                ui.label("Foreground:");
                Self::color_picker(ui, &mut self.fg_color);
            });

            ui.horizontal(|ui| {
                ui.label("Background:");
                Self::color_picker(ui, &mut self.bg_color);
            });

            // Color Presets
            ui.label("Quick Presets:");
            ui.horizontal_wrapped(|ui| {
                for preset in COLOR_PRESETS {
                    if ui.button(preset.name).clicked() {
                        self.fg_color = preset.fg;
                        self.bg_color = preset.bg;
                        self.status_message = format!("Applied {} preset", preset.name);
                    }
                }
            });
        });

        ui.add_space(10.0);

        // Gradient Section
        ui.group(|ui| {
            ui.label("🌈 Gradient Options:");
            
            ui.checkbox(&mut self.use_gradient, "Enable Gradient");
            
            if self.use_gradient {
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_salt("gradient_type")
                        .selected_text(format!("{:?}", self.gradient_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.gradient_type, GradientType::Horizontal, "Horizontal");
                            ui.selectable_value(&mut self.gradient_type, GradientType::Vertical, "Vertical");
                            ui.selectable_value(&mut self.gradient_type, GradientType::Radial, "Radial");
                            ui.selectable_value(&mut self.gradient_type, GradientType::Diagonal, "Diagonal");
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("End Color:");
                    Self::color_picker(ui, &mut self.gradient_color);
                });
            }
        });

        ui.add_space(10.0);

        // Module Style Section
        ui.group(|ui| {
            ui.label("✨ Module Style:");
            
            ui.horizontal(|ui| {
                ui.label("Shape:");
                egui::ComboBox::from_id_salt("module_style")
                    .selected_text(format!("{:?}", self.module_style))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.module_style, ModuleStyle::Square, "Square");
                        ui.selectable_value(&mut self.module_style, ModuleStyle::Circle, "Circle");
                        ui.selectable_value(&mut self.module_style, ModuleStyle::RoundedSquare, "Rounded Square");
                        ui.selectable_value(&mut self.module_style, ModuleStyle::Dots, "Dots");
                    });
            });

            ui.checkbox(&mut self.use_rounded_corners, "Extra Rounded Corners");
            
            if self.use_rounded_corners {
                ui.horizontal(|ui| {
                    ui.label("Corner Radius:");
                    ui.add(egui::Slider::new(&mut self.corner_radius, 0.0..=1.0));
                });
            }
        });

        ui.add_space(10.0);

        // Eye Style Section
        ui.group(|ui| {
            ui.label("👁️ Finder Pattern (Eyes):");
            
            ui.horizontal(|ui| {
                ui.label("Style:");
                egui::ComboBox::from_id_salt("eye_style")
                    .selected_text(format!("{:?}", self.eye_style))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.eye_style, EyeStyle::Standard, "Standard");
                        ui.selectable_value(&mut self.eye_style, EyeStyle::Circle, "Circle");
                        ui.selectable_value(&mut self.eye_style, EyeStyle::RoundedSquare, "Rounded");
                        ui.selectable_value(&mut self.eye_style, EyeStyle::Flower, "Flower");
                        ui.selectable_value(&mut self.eye_style, EyeStyle::Diamond, "Diamond");
                    });
            });

            ui.checkbox(&mut self.use_custom_eye_color, "Custom Eye Color");
            
            if self.use_custom_eye_color {
                ui.horizontal(|ui| {
                    ui.label("Eye Color:");
                    Self::color_picker(ui, &mut self.eye_color);
                });
            }
        });
    }

    /// Render advanced settings tab
    fn render_advanced_tab(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("🔍 Opacity Controls:");
            
            ui.horizontal(|ui| {
                ui.label("QR Opacity:");
                ui.add(egui::Slider::new(&mut self.qr_opacity, 0.0..=1.0));
            });
            
            ui.label("Use lower opacity for watermark effects or blending with backgrounds");
        });
    }

    /// Render images tab
    fn render_images_tab(&mut self, ui: &mut egui::Ui) {
        // Logo Section
        ui.group(|ui| {
            ui.label("🎯 Logo Overlay:");
            
            ui.horizontal(|ui| {
                if ui.button("📂 Select Logo").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp"])
                        .pick_file() 
                    {
                        match image::open(&path) {
                            Ok(img) => {
                                self.logo_image = Some(img);
                                self.logo_path = Some(path.clone());
                                self.status_message = format!("Logo loaded: {}", path.display());
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to load logo: {}", e);
                            }
                        }
                    }
                }

                if self.logo_path.is_some() {
                    if ui.button("❌ Clear").clicked() {
                        self.logo_path = None;
                        self.logo_image = None;
                        self.status_message = "Logo cleared".to_string();
                    }
                }
            });

            if let Some(path) = &self.logo_path {
                ui.label(format!("📎 {}", path.file_name().unwrap().to_string_lossy()));
                
                ui.horizontal(|ui| {
                    ui.label("Logo Size:");
                    ui.add(egui::Slider::new(&mut self.logo_size, 0.05..=0.35).suffix("%"));
                });
                
                ui.label("⚠️ Keep logo under 30% for best scannability");
            } else {
                ui.label("No logo selected. A logo will be centered on the QR code.");
            }
        });

        ui.add_space(10.0);

        // Background Image Section
        ui.group(|ui| {
            ui.label("🖼️ Background Image:");
            
            ui.horizontal(|ui| {
                if ui.button("📂 Select Background").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp"])
                        .pick_file() 
                    {
                        match image::open(&path) {
                            Ok(img) => {
                                self.bg_image = Some(img);
                                self.bg_image_path = Some(path.clone());
                                self.status_message = format!("Background loaded: {}", path.display());
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to load background: {}", e);
                            }
                        }
                    }
                }

                if self.bg_image_path.is_some() {
                    if ui.button("❌ Clear").clicked() {
                        self.bg_image_path = None;
                        self.bg_image = None;
                        self.status_message = "Background cleared".to_string();
                    }
                }
            });

            if let Some(path) = &self.bg_image_path {
                ui.label(format!("📎 {}", path.file_name().unwrap().to_string_lossy()));
                
                ui.horizontal(|ui| {
                    ui.label("Opacity:");
                    ui.add(egui::Slider::new(&mut self.bg_image_opacity, 0.0..=1.0));
                });
                
                ui.label("Background image will be blended behind the QR code");
            } else {
                ui.label("No background image selected.");
            }
        });
    }

    /// Render the preview panel UI
    fn render_preview(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Preview");
        ui.separator();

        if let Some(texture) = &self.preview_texture {
            // Get all available space
            let available = ui.available_size();
            
            // Use most of the available space (with some padding)
            let size = (available.x.min(available.y) * 0.9).max(300.0);
            
            // Center and display
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Show the QR code with white background
                let rect_pos = ui.cursor().min;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect_pos, egui::vec2(size, size)),
                    0.0,
                    egui::Color32::WHITE,
                );
                
                ui.image((texture.id(), egui::vec2(size, size)));
                
                ui.add_space(10.0);
                ui.label(format!("📐 {} x {} pixels", texture.size()[0], texture.size()[1]));
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("⏳ Loading...");
                ui.add_space(20.0);
                ui.label("Your QR code will appear here");
                ui.add_space(20.0);
                
                if ui.button("🔄 Generate Now").clicked() {
                    self.generate_preview(ctx);
                }
            });
        }
    }

    /// Helper function to render a color picker
    fn color_picker(ui: &mut egui::Ui, color: &mut [u8; 3]) {
        let mut color_f32 = [
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
        ];

        if ui.color_edit_button_rgb(&mut color_f32).changed() {
            color[0] = (color_f32[0] * 255.0) as u8;
            color[1] = (color_f32[1] * 255.0) as u8;
            color[2] = (color_f32[2] * 255.0) as u8;
        }
    }

    /// Generate QR code preview and update texture
    fn generate_preview(&mut self, ctx: &egui::Context) {
        if self.qr_text.is_empty() {
            self.status_message = "⚠️ Please enter text for the QR code".to_string();
            return;
        }

        self.status_message = "🔄 Generating QR code...".to_string();
        
        match self.generate_qr_image() {
            Ok(img) => {
                let width = img.width() as usize;
                let height = img.height() as usize;
                
                // Convert RGBA image to Color32 array for egui
                let pixels: Vec<egui::Color32> = img.pixels()
                    .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                    .collect();
                
                let color_image = egui::ColorImage {
                    size: [width, height],
                    pixels,
                };
                
                self.preview_texture = Some(ctx.load_texture(
                    "qr_preview",
                    color_image,
                    egui::TextureOptions::NEAREST,
                ));
                
                self.status_message = format!("✅ QR code generated successfully! ({}x{})", width, height);
            }
            Err(e) => {
                self.status_message = format!("❌ Error: {}", e);
            }
        }
    }

    /// Generate QR code image with current settings
    fn generate_qr_image(&self) -> Result<RgbaImage, String> {
        // Create QR code with error correction
        let code = QrCode::with_error_correction_level(&self.qr_text, self.ec_level.to_ec_level())
            .map_err(|e| format!("Failed to create QR code: {}", e))?;

        // Get QR code matrix
        let matrix = code.to_colors();
        let qr_width = code.width();

        // Calculate module size
        let module_size = (self.size - 2 * self.border * (self.size / qr_width as u32)) / qr_width as u32;
        let actual_qr_size = module_size * qr_width as u32;
        let total_size = actual_qr_size + 2 * self.border * module_size;

        // Create base image or use background image
        let mut image: RgbaImage = if let Some(bg_img) = &self.bg_image {
            // Resize background image to match QR code size
            let resized = bg_img.resize_exact(total_size, total_size, imageops::FilterType::Lanczos3);
            let mut img_buffer = resized.to_rgba8();
            
            // Apply opacity to background
            if self.bg_image_opacity < 1.0 {
                for pixel in img_buffer.pixels_mut() {
                    pixel[3] = (pixel[3] as f32 * self.bg_image_opacity) as u8;
                }
            }
            
            // Blend with background color
            let mut base = ImageBuffer::new(total_size, total_size);
            for pixel in base.pixels_mut() {
                *pixel = Rgba([self.bg_color[0], self.bg_color[1], self.bg_color[2], 255]);
            }
            
            imageops::overlay(&mut base, &img_buffer, 0, 0);
            base
        } else {
            let mut img = ImageBuffer::new(total_size, total_size);
            // Fill background
            for pixel in img.pixels_mut() {
                *pixel = Rgba([self.bg_color[0], self.bg_color[1], self.bg_color[2], 255]);
            }
            img
        };

        // Identify eye positions (finder patterns at corners)
        let eye_positions = vec![
            (0, 0),                           // Top-left
            (qr_width - 7, 0),                // Top-right
            (0, qr_width - 7),                // Bottom-left
        ];

        // Draw QR code modules
        let offset = self.border * module_size;
        
        for y in 0..qr_width {
            for x in 0..qr_width {
                let is_dark = matches!(matrix[y * qr_width + x], qrcode::Color::Dark);
                
                if is_dark {
                    let px = offset + x as u32 * module_size;
                    let py = offset + y as u32 * module_size;
                    
                    // Check if this module is part of an eye
                    let is_eye = eye_positions.iter().any(|(ex, ey)| {
                        x >= *ex && x < ex + 7 && y >= *ey && y < ey + 7
                    });
                    
                    if is_eye {
                        self.draw_eye_module(&mut image, x, y, px, py, module_size, &eye_positions, qr_width);
                    } else {
                        self.draw_data_module(&mut image, x, y, px, py, module_size, qr_width);
                    }
                }
            }
        }

        // Apply logo overlay if present
        if let Some(logo_img) = &self.logo_image {
            self.apply_logo_overlay(&mut image, logo_img, qr_width, module_size, offset)?;
        }

        // Apply overall opacity if needed
        if self.qr_opacity < 1.0 {
            for pixel in image.pixels_mut() {
                // Only apply opacity to non-background pixels
                if pixel[0] != self.bg_color[0] || pixel[1] != self.bg_color[1] || pixel[2] != self.bg_color[2] {
                    pixel[3] = (pixel[3] as f32 * self.qr_opacity) as u8;
                }
            }
        }

        Ok(image)
    }

    /// Draw a data module (non-eye module)
    fn draw_data_module(&self, image: &mut RgbaImage, _x: usize, _y: usize, px: u32, py: u32, size: u32, _qr_width: usize) {
        let color = if self.use_gradient {
            self.get_gradient_color(px, py, image.width(), image.height())
        } else {
            Rgba([self.fg_color[0], self.fg_color[1], self.fg_color[2], 255])
        };

        match self.module_style {
            ModuleStyle::Square => {
                self.draw_square(image, px, py, size, color);
            }
            ModuleStyle::Circle => {
                self.draw_circle(image, px, py, size, color);
            }
            ModuleStyle::RoundedSquare => {
                self.draw_rounded_square(image, px, py, size, color);
            }
            ModuleStyle::Dots => {
                self.draw_dot(image, px, py, size, color);
            }
        }
    }

    /// Draw an eye module (finder pattern)
    fn draw_eye_module(&self, image: &mut RgbaImage, x: usize, y: usize, px: u32, py: u32, 
                       size: u32, eye_positions: &[(usize, usize)], _qr_width: usize) {
        let color = if self.use_custom_eye_color {
            Rgba([self.eye_color[0], self.eye_color[1], self.eye_color[2], 255])
        } else if self.use_gradient {
            self.get_gradient_color(px, py, image.width(), image.height())
        } else {
            Rgba([self.fg_color[0], self.fg_color[1], self.fg_color[2], 255])
        };

        // Find which eye this belongs to and relative position
        for (ex, ey) in eye_positions {
            if x >= *ex && x < ex + 7 && y >= *ey && y < ey + 7 {
                let rel_x = x - ex;
                let rel_y = y - ey;
                
                match self.eye_style {
                    EyeStyle::Standard => {
                        self.draw_square(image, px, py, size, color);
                    }
                    EyeStyle::Circle => {
                        self.draw_eye_circle(image, px, py, size, color, rel_x, rel_y);
                    }
                    EyeStyle::RoundedSquare => {
                        self.draw_eye_rounded(image, px, py, size, color, rel_x, rel_y);
                    }
                    EyeStyle::Flower => {
                        self.draw_eye_flower(image, px, py, size, color, rel_x, rel_y);
                    }
                    EyeStyle::Diamond => {
                        self.draw_eye_diamond(image, px, py, size, color, rel_x, rel_y);
                    }
                }
                break;
            }
        }
    }

    /// Get gradient color based on position
    fn get_gradient_color(&self, x: u32, y: u32, width: u32, height: u32) -> Rgba<u8> {
        let t = match self.gradient_type {
            GradientType::Horizontal => x as f32 / width as f32,
            GradientType::Vertical => y as f32 / height as f32,
            GradientType::Diagonal => (x + y) as f32 / (width + height) as f32,
            GradientType::Radial => {
                let cx = width as f32 / 2.0;
                let cy = height as f32 / 2.0;
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let max_dist = (cx * cx + cy * cy).sqrt();
                (dist / max_dist).min(1.0)
            }
        };

        // Interpolate between fg_color and gradient_color
        let r = (self.fg_color[0] as f32 * (1.0 - t) + self.gradient_color[0] as f32 * t) as u8;
        let g = (self.fg_color[1] as f32 * (1.0 - t) + self.gradient_color[1] as f32 * t) as u8;
        let b = (self.fg_color[2] as f32 * (1.0 - t) + self.gradient_color[2] as f32 * t) as u8;

        Rgba([r, g, b, 255])
    }

    /// Apply logo overlay to the center of the QR code
    fn apply_logo_overlay(&self, image: &mut RgbaImage, logo: &DynamicImage, 
                          qr_width: usize, module_size: u32, offset: u32) -> Result<(), String> {
        // Calculate logo size
        let qr_size = qr_width as u32 * module_size;
        let logo_size = (qr_size as f32 * self.logo_size) as u32;
        
        // Resize logo
        let resized_logo = logo.resize_exact(logo_size, logo_size, imageops::FilterType::Lanczos3);
        let logo_rgba = resized_logo.to_rgba8();
        
        // Calculate center position
        let center_x = offset + (qr_size - logo_size) / 2;
        let center_y = offset + (qr_size - logo_size) / 2;
        
        // Overlay logo with alpha blending
        imageops::overlay(image, &logo_rgba, center_x as i64, center_y as i64);
        
        Ok(())
    }

    /// Draw eye patterns with different styles
    fn draw_eye_circle(&self, image: &mut RgbaImage, px: u32, py: u32, size: u32, color: Rgba<u8>, rel_x: usize, rel_y: usize) {
        // Create circular eye pattern
        if (rel_x <= 1 || rel_x >= 5 || rel_y <= 1 || rel_y >= 5) && !(rel_x == 3 && rel_y == 3) {
            self.draw_circle(image, px, py, size, color);
        } else if rel_x == 3 && rel_y == 3 {
            self.draw_circle(image, px, py, size, color);
        }
    }

    fn draw_eye_rounded(&self, image: &mut RgbaImage, px: u32, py: u32, size: u32, color: Rgba<u8>, _rel_x: usize, _rel_y: usize) {
        self.draw_rounded_square(image, px, py, size, color);
    }

    fn draw_eye_flower(&self, image: &mut RgbaImage, px: u32, py: u32, size: u32, color: Rgba<u8>, rel_x: usize, rel_y: usize) {
        // Create a flower-like pattern
        let is_outer = rel_x == 0 || rel_x == 6 || rel_y == 0 || rel_y == 6;
        let is_inner = (rel_x >= 2 && rel_x <= 4) && (rel_y >= 2 && rel_y <= 4);
        
        if is_outer || is_inner {
            if (rel_x + rel_y) % 2 == 0 {
                self.draw_circle(image, px, py, size, color);
            } else {
                self.draw_rounded_square(image, px, py, size, color);
            }
        }
    }

    fn draw_eye_diamond(&self, image: &mut RgbaImage, px: u32, py: u32, size: u32, color: Rgba<u8>, rel_x: usize, rel_y: usize) {
        // Create diamond shape by drawing only certain positions
        let cx = 3;
        let cy = 3;
        let dist = ((rel_x as i32 - cx).abs() + (rel_y as i32 - cy).abs()) as usize;
        
        if dist == 3 || dist == 1 {
            self.draw_square(image, px, py, size, color);
        }
    }

    /// Draw a square module
    fn draw_square(&self, image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
        for dy in 0..size {
            for dx in 0..size {
                if x + dx < image.width() && y + dy < image.height() {
                    image.put_pixel(x + dx, y + dy, color);
                }
            }
        }
    }

    /// Draw a circular module
    fn draw_circle(&self, image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
        let radius = size as f32 / 2.0;
        let center_x = x as f32 + radius;
        let center_y = y as f32 + radius;

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

    /// Draw a rounded square module
    fn draw_rounded_square(&self, image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
        let radius = if self.use_rounded_corners {
            (size as f32 * self.corner_radius) as u32
        } else {
            (size as f32 * 0.2) as u32
        };

        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;

                // Check if we're in a corner region
                let in_corner = (dx < radius && dy < radius) ||
                               (dx >= size - radius && dy < radius) ||
                               (dx < radius && dy >= size - radius) ||
                               (dx >= size - radius && dy >= size - radius);

                if in_corner {
                    // Calculate distance from nearest corner
                    let corner_x = if dx < radius { radius } else { size - radius };
                    let corner_y = if dy < radius { radius } else { size - radius };
                    
                    let dist = ((dx as f32 - corner_x as f32).powi(2) + 
                               (dy as f32 - corner_y as f32).powi(2)).sqrt();
                    
                    if dist <= radius as f32 && px < image.width() && py < image.height() {
                        image.put_pixel(px, py, color);
                    }
                } else if px < image.width() && py < image.height() {
                    image.put_pixel(px, py, color);
                }
            }
        }
    }

    /// Draw a dot-style module
    fn draw_dot(&self, image: &mut RgbaImage, x: u32, y: u32, size: u32, color: Rgba<u8>) {
        let radius = size as f32 * 0.35;
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

    /// Save QR code to PNG file
    fn save_qr_code(&mut self) {
        if self.qr_text.is_empty() {
            self.status_message = "⚠️ Please enter text for the QR code".to_string();
            return;
        }

        // Generate filename with timestamp
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let default_filename = format!("qrcode_{}.png", timestamp);

        // Open file dialog
        let file = rfd::FileDialog::new()
            .set_file_name(&default_filename)
            .add_filter("PNG Image", &["png"])
            .save_file();

        if let Some(path) = file {
            match self.generate_qr_image() {
                Ok(image) => {
                    match image.save(&path) {
                        Ok(_) => {
                            self.status_message = format!("✅ Saved to: {}", path.display());
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Failed to save: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.status_message = format!("❌ Error generating QR code: {}", e);
                }
            }
        } else {
            self.status_message = "Save cancelled".to_string();
        }
    }

    /// Save current configuration as a preset
    fn save_preset(&mut self) {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let default_filename = format!("qr_preset_{}.json", timestamp);

        let file = rfd::FileDialog::new()
            .set_file_name(&default_filename)
            .add_filter("JSON Preset", &["json"])
            .save_file();

        if let Some(path) = file {
            match serde_json::to_string_pretty(self) {
                Ok(json) => {
                    match std::fs::write(&path, json) {
                        Ok(_) => {
                            self.status_message = format!("✅ Preset saved to: {}", path.display());
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Failed to save preset: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.status_message = format!("❌ Failed to serialize preset: {}", e);
                }
            }
        } else {
            self.status_message = "Save cancelled".to_string();
        }
    }

    /// Load a configuration preset
    fn load_preset(&mut self, ctx: &egui::Context) {
        let file = rfd::FileDialog::new()
            .add_filter("JSON Preset", &["json"])
            .pick_file();

        if let Some(path) = file {
            match std::fs::read_to_string(&path) {
                Ok(json) => {
                    match serde_json::from_str::<QrCodeApp>(&json) {
                        Ok(mut loaded) => {
                            // Preserve runtime-only fields
                            loaded.preview_texture = None;
                            loaded.status_message = format!("✅ Preset loaded from: {}", path.display());
                            loaded.selected_tab = self.selected_tab;
                            
                            // Try to reload images if paths exist
                            if let Some(logo_path) = &loaded.logo_path {
                                if let Ok(img) = image::open(logo_path) {
                                    loaded.logo_image = Some(img);
                                }
                            }
                            if let Some(bg_path) = &loaded.bg_image_path {
                                if let Ok(img) = image::open(bg_path) {
                                    loaded.bg_image = Some(img);
                                }
                            }
                            
                            *self = loaded;
                            
                            // Auto-generate preview
                            self.generate_preview(ctx);
                        }
                        Err(e) => {
                            self.status_message = format!("❌ Failed to parse preset: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.status_message = format!("❌ Failed to read preset file: {}", e);
                }
            }
        } else {
            self.status_message = "Load cancelled".to_string();
        }
    }
}