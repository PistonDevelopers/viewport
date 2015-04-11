#![deny(missing_docs)]

//! A library for storing viewport information

/// Stores viewport information.
#[derive(Copy, Clone)]
pub struct Viewport {
    /// Viewport in pixels.
    pub rect: [i32; 4],
    /// The size of frame buffer in pixels.
    pub draw_size: [u32; 2],
    /// The size of window in points.
    pub window_size: [u32; 2],
}

