//! User interface modules
//!
//! Handles all GUI rendering logic including controls, tabs, and preview display.

pub mod tabs;
pub mod preview;
pub mod helpers;

// Re-export main functions for convenience
pub use preview::render_preview;
pub use tabs::render_controls;
