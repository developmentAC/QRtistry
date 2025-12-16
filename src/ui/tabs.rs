//! Tab rendering for control panel
//!
//! Organizes all settings into four logical tabs:
//! - Basic: Content, dimensions, error correction
//! - Style: Colors, gradients, module/eye styles
//! - Advanced: Opacity controls
//! - Images: Logo and background image integration

use eframe::egui;

use crate::app::QrCodeApp;
use crate::types::*;
use crate::ui::helpers;

/// Main control panel renderer with tab selection
///
/// Displays tab buttons and renders the selected tab's content.
///
/// # Arguments
/// * `app` - Application state
/// * `ui` - egui UI context
/// * `_ctx` - egui context (unused here)
pub fn render_controls(app: &mut QrCodeApp, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.heading("Settings");
    ui.add_space(10.0);

    // === Tab Selection Bar ===
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.selected_tab, TabSelection::Basic, "📝 Basic");
        ui.selectable_value(&mut app.selected_tab, TabSelection::Style, "🎨 Style");
        ui.selectable_value(&mut app.selected_tab, TabSelection::Advanced, "⚙️ Advanced");
        ui.selectable_value(&mut app.selected_tab, TabSelection::Images, "🖼️ Images");
    });

    ui.separator();
    ui.add_space(10.0);

    // === Render Selected Tab ===
    match app.selected_tab {
        TabSelection::Basic => render_basic_tab(app, ui),
        TabSelection::Style => render_style_tab(app, ui),
        TabSelection::Advanced => render_advanced_tab(app, ui),
        TabSelection::Images => render_images_tab(app, ui),
    }
}

// ============================================================================
// Basic Tab
// ============================================================================

/// Render the Basic settings tab
///
/// Contains essential QR code settings:
/// - Text content input
/// - Size and border dimensions  
/// - Error correction level
fn render_basic_tab(app: &mut QrCodeApp, ui: &mut egui::Ui) {
    // === QR Code Content Section ===
    ui.group(|ui| {
        ui.label("📝 QR Code Content:");
        ui.add(
            egui::TextEdit::multiline(&mut app.qr_text)
                .desired_width(f32::INFINITY)
                .desired_rows(8)
        );
        ui.label(format!("Characters: {}", app.qr_text.len()));
        
        if app.qr_text.len() > 500 {
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠️ Long text may require high error correction"
            );
        }
    });

    ui.add_space(10.0);

    // === Dimensions Section ===
    ui.group(|ui| {
        ui.label("📐 Dimensions:");
        ui.add_space(5.0);
        
        // Output size slider
        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut app.size, 128..=2048).suffix(" px"));
        });
        
        ui.add_space(8.0);
        
        // Border (quiet zone) slider
        ui.horizontal(|ui| {
            ui.label("Border:");
            ui.add(egui::Slider::new(&mut app.border, 0..=10).suffix(" modules"));
        });
        
        ui.add_space(5.0);
        ui.label("💡 Tip: Use 2-4 modules for reliable scanning");
    });

    ui.add_space(10.0);

    // === Error Correction Section ===
    ui.group(|ui| {
        ui.label("🛡️ Error Correction:");
        ui.add_space(5.0);
        
        // Error correction level buttons
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut app.ec_level, ErrorCorrectionLevel::Low, "Low");
            ui.selectable_value(&mut app.ec_level, ErrorCorrectionLevel::Medium, "Medium");
            ui.selectable_value(&mut app.ec_level, ErrorCorrectionLevel::Quartile, "Quartile");
            ui.selectable_value(&mut app.ec_level, ErrorCorrectionLevel::High, "High");
        });
        
        ui.add_space(5.0);
        
        // Explanation
        let explanation = match app.ec_level {
            ErrorCorrectionLevel::Low => "Low (7%): Smallest QR code, minimal decoration",
            ErrorCorrectionLevel::Medium => "Medium (15%): Balanced, recommended for most uses",
            ErrorCorrectionLevel::Quartile => "Quartile (25%): Good for styled QR codes",
            ErrorCorrectionLevel::High => "High (30%): Maximum resilience, best for logos",
        };
        ui.label(explanation);
    });
}

// ============================================================================
// Style Tab
// ============================================================================

/// Render the Style settings tab
///
/// Contains visual customization options:
/// - Foreground/background colors
/// - Color presets
/// - Gradient configuration
/// - Module shape styles
/// - Eye (finder pattern) styles
fn render_style_tab(app: &mut QrCodeApp, ui: &mut egui::Ui) {
    // === Color Section ===
    ui.group(|ui| {
        ui.label("🎨 Colors:");
        
        // Foreground color picker
        ui.horizontal(|ui| {
            ui.label("Foreground:");
            helpers::color_picker(ui, &mut app.fg_color);
            ui.label(format!(
                "RGB({}, {}, {})",
                app.fg_color[0], app.fg_color[1], app.fg_color[2]
            ));
        });

        // Background color picker
        ui.horizontal(|ui| {
            ui.label("Background:");
            helpers::color_picker(ui, &mut app.bg_color);
            ui.label(format!(
                "RGB({}, {}, {})",
                app.bg_color[0], app.bg_color[1], app.bg_color[2]
            ));
        });

        // Color Presets
        ui.add_space(5.0);
        ui.label("Quick Presets:");
        ui.horizontal_wrapped(|ui| {
            for preset in COLOR_PRESETS {
                if ui.button(preset.name).clicked() {
                    app.fg_color = preset.fg;
                    app.bg_color = preset.bg;
                    app.status_message = format!("Applied {} preset", preset.name);
                }
            }
        });
    });

    ui.add_space(10.0);

    // === Gradient Section ===
    ui.group(|ui| {
        ui.label("🌈 Gradient Options:");
        
        ui.checkbox(&mut app.use_gradient, "Enable Gradient");
        
        if app.use_gradient {
            ui.add_space(5.0);
            
            // Gradient type selector
            ui.horizontal(|ui| {
                ui.label("Type:");
                egui::ComboBox::from_id_salt("gradient_type")
                    .selected_text(format!("{:?}", app.gradient_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.gradient_type, GradientType::Horizontal, "Horizontal");
                        ui.selectable_value(&mut app.gradient_type, GradientType::Vertical, "Vertical");
                        ui.selectable_value(&mut app.gradient_type, GradientType::Radial, "Radial");
                        ui.selectable_value(&mut app.gradient_type, GradientType::Diagonal, "Diagonal");
                    });
            });

            // Gradient end color picker
            ui.horizontal(|ui| {
                ui.label("End Color:");
                helpers::color_picker(ui, &mut app.gradient_color);
            });
            
            ui.add_space(3.0);
            ui.label("💡 Gradients blend from foreground to end color");
        }
    });

    ui.add_space(10.0);

    // === Module Style Section ===
    ui.group(|ui| {
        ui.label("✨ Module Style:");
        
        // Module shape selector
        ui.horizontal(|ui| {
            ui.label("Shape:");
            egui::ComboBox::from_id_salt("module_style")
                .selected_text(format!("{:?}", app.module_style))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.module_style, ModuleStyle::Square, "Square");
                    ui.selectable_value(&mut app.module_style, ModuleStyle::Circle, "Circle");
                    ui.selectable_value(&mut app.module_style, ModuleStyle::RoundedSquare, "Rounded Square");
                    ui.selectable_value(&mut app.module_style, ModuleStyle::Dots, "Dots");
                });
        });

        ui.add_space(5.0);
        
        // Extra rounding option
        ui.checkbox(&mut app.use_rounded_corners, "Extra Rounded Corners");
        
        if app.use_rounded_corners {
            ui.horizontal(|ui| {
                ui.label("Corner Radius:");
                ui.add(egui::Slider::new(&mut app.corner_radius, 0.0..=1.0));
            });
        }
        
        ui.add_space(3.0);
        ui.label("💡 Square and Rounded Square are most reliable for scanning");
    });

    ui.add_space(10.0);

    // === Eye Style Section ===
    ui.group(|ui| {
        ui.label("👁️ Finder Pattern (Eyes):");
        
        // Eye style selector
        ui.horizontal(|ui| {
            ui.label("Style:");
            egui::ComboBox::from_id_salt("eye_style")
                .selected_text(format!("{:?}", app.eye_style))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.eye_style, EyeStyle::Standard, "Standard");
                    ui.selectable_value(&mut app.eye_style, EyeStyle::Circle, "Circle");
                    ui.selectable_value(&mut app.eye_style, EyeStyle::RoundedSquare, "Rounded");
                    ui.selectable_value(&mut app.eye_style, EyeStyle::Flower, "Flower");
                    ui.selectable_value(&mut app.eye_style, EyeStyle::Diamond, "Diamond");
                });
        });

        ui.add_space(5.0);
        
        // Custom eye color option
        ui.checkbox(&mut app.use_custom_eye_color, "Custom Eye Color");
        
        if app.use_custom_eye_color {
            ui.horizontal(|ui| {
                ui.label("Eye Color:");
                helpers::color_picker(ui, &mut app.eye_color);
            });
        }
        
        ui.add_space(3.0);
        ui.label("💡 Eyes are the three corner squares that help scanners locate the QR code");
    });
}

// ============================================================================
// Advanced Tab
// ============================================================================

/// Render the Advanced settings tab
///
/// Contains fine-tuning options:
/// - Overall QR code opacity
fn render_advanced_tab(app: &mut QrCodeApp, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label("🔍 Opacity Controls:");
        
        // QR opacity slider
        ui.horizontal(|ui| {
            ui.label("QR Opacity:");
            ui.add(egui::Slider::new(&mut app.qr_opacity, 0.0..=1.0));
        });
        
        ui.add_space(5.0);
        ui.label("Use lower opacity for watermark effects or subtle integration with backgrounds");
        
        if app.qr_opacity < 0.5 {
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠️ Low opacity may reduce scannability"
            );
        }
    });
    
    ui.add_space(10.0);
    
    // Placeholder for future advanced settings
    ui.group(|ui| {
        ui.label("ℹ️ About:");
        ui.label("This tab is reserved for advanced features.");
        ui.label("More options may be added in future versions:");
        ui.label("• Fine-tune module spacing");
        ui.label("• Custom timing patterns");
        ui.label("• Advanced color blending modes");
    });
}

// ============================================================================
// Images Tab
// ============================================================================

/// Render the Images tab
///
/// Handles image integration:
/// - Logo overlay (center of QR code)
/// - Background image blending
fn render_images_tab(app: &mut QrCodeApp, ui: &mut egui::Ui) {
    // === Logo Section ===
    ui.group(|ui| {
        ui.label("🎯 Logo Overlay:");
        
        // Logo file selection buttons
        ui.horizontal(|ui| {
            if ui.button("📂 Select Logo").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp"])
                    .pick_file() 
                {
                    match image::open(&path) {
                        Ok(img) => {
                            app.logo_image = Some(img);
                            app.logo_path = Some(path.clone());
                            app.status_message = format!("Logo loaded: {}", path.display());
                        }
                        Err(e) => {
                            app.status_message = format!("Failed to load logo: {}", e);
                        }
                    }
                }
            }

            if app.logo_path.is_some() {
                if ui.button("❌ Clear").clicked() {
                    app.logo_path = None;
                    app.logo_image = None;
                    app.status_message = "Logo cleared".to_string();
                }
            }
        });

        // Logo settings (only show if logo is loaded)
        if let Some(path) = &app.logo_path {
            ui.add_space(5.0);
            ui.label(format!("📎 {}", path.file_name().unwrap().to_string_lossy()));
            
            // Logo size slider
            ui.horizontal(|ui| {
                ui.label("Logo Size:");
                ui.add(egui::Slider::new(&mut app.logo_size, 0.05..=0.35)
                    .suffix("%")
                    .custom_formatter(|n, _| format!("{:.0}%", n * 100.0))
                );
            });
            
            ui.add_space(5.0);
            
            // Warning about scannability
            if app.logo_size > 0.25 {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    "⚠️ Large logos may reduce scannability"
                );
            } else {
                ui.label("💡 Keep logo under 30% for best scannability");
            }
            
            if app.ec_level != ErrorCorrectionLevel::High {
                ui.colored_label(
                    egui::Color32::LIGHT_BLUE,
                    "💡 Tip: Use High error correction with logos"
                );
            }
        } else {
            ui.add_space(5.0);
            ui.label("No logo selected. A logo will be centered on the QR code.");
        }
    });

    ui.add_space(10.0);

    // === Background Image Section ===
    ui.group(|ui| {
        ui.label("🖼️ Background Image:");
        
        // Background file selection buttons
        ui.horizontal(|ui| {
            if ui.button("📂 Select Background").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp"])
                    .pick_file() 
                {
                    match image::open(&path) {
                        Ok(img) => {
                            app.bg_image = Some(img);
                            app.bg_image_path = Some(path.clone());
                            app.status_message = format!("Background loaded: {}", path.display());
                        }
                        Err(e) => {
                            app.status_message = format!("Failed to load background: {}", e);
                        }
                    }
                }
            }

            if app.bg_image_path.is_some() {
                if ui.button("❌ Clear").clicked() {
                    app.bg_image_path = None;
                    app.bg_image = None;
                    app.status_message = "Background cleared".to_string();
                }
            }
        });

        // Background settings (only show if image is loaded)
        if let Some(path) = &app.bg_image_path {
            ui.add_space(5.0);
            ui.label(format!("📎 {}", path.file_name().unwrap().to_string_lossy()));
            
            // Background opacity slider
            ui.horizontal(|ui| {
                ui.label("Opacity:");
                ui.add(egui::Slider::new(&mut app.bg_image_opacity, 0.0..=1.0)
                    .custom_formatter(|n, _| format!("{:.0}%", n * 100.0))
                );
            });
            
            ui.add_space(5.0);
            ui.label("Background image will be resized and blended behind the QR code");
            
            if app.bg_image_opacity > 0.7 {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    "⚠️ High background opacity may reduce contrast"
                );
            }
        } else {
            ui.add_space(5.0);
            ui.label("No background image selected.");
        }
    });
}
