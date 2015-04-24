#![deny(missing_docs)]

//! A library for storing viewport information

extern crate float;

use float::*;

/// Stores viewport information.
#[derive(Copy, Clone)]
pub struct Viewport {
    /// Viewport in pixels.
    /// ```[x, y, width height]``` where ```(x, y)``` is lower left corner.
    pub rect: [i32; 4],
    /// The size of frame buffer in pixels.
    pub draw_size: [u32; 2],
    /// The size of window in points.
    pub window_size: [u32; 2],
}

impl Viewport {
    /// Computes absolute transform for 2D graphics,
    /// which uses a row major 2x3 matrix.
    ////
    /// The origin is in the upper left corner of the viewport rectangle.
    /// The x axis points to the right, and the y axis points down.
    /// The units are in points (window coordinates).
    ///
    /// It is assumed that the underlying coordinate system is normalized
    /// with the origin in the center, such that ```(-1.0, 1.0)``` in the
    /// underlying coordinate system corresponds to the
    /// upper left corner of the viewport.
    pub fn abs_transform<T: Float>(&self) -> [[T; 3]; 2] {
        let (dw, dh) = (self.draw_size[0] as f64, self.draw_size[1] as f64);
        let (ww, wh) = (self.window_size[0] as f64, self.window_size[1] as f64);
        let sx = 2.0 * (dw / ww) / self.rect[2] as f64;
        let sy = -2.0 * (dh / wh) / self.rect[3] as f64;
        let f = |x| FromPrimitive::from_f64(x);
        [
            [f(sx), f(0.0), f(-1.0)],
            [f(0.0), f(sy), f(1.0)]
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs() {
        let transform = |mat: [[f64; 3]; 2], (px, py)| -> (f64, f64) {
            (mat[0][0] * px + mat[0][1] * py + mat[0][2],
            mat[1][0] * px + mat[1][1] * py + mat[1][2])
         };

        let viewport = Viewport {
            rect: [0, 0, 100, 200],
            draw_size: [100, 200],
            window_size: [100, 200],
        };
        let abs = viewport.abs_transform();
        assert_eq!(abs, [
                [0.02, 0.0, -1.0],
                [0.0, -0.01, 1.0]
            ]);
        assert_eq!(transform(abs, (0.0, 0.0)), (-1.0, 1.0));
        assert_eq!(transform(abs, (50.0, 100.0)), (0.0, 0.0));
        assert_eq!(transform(abs, (100.0, 200.0)), (1.0, -1.0));

        let viewport = Viewport {
            rect: [10, 10, 80, 80],
            draw_size: [100, 100],
            window_size: [50, 50],
        };
        let abs = viewport.abs_transform();
        assert_eq!(abs, [
                [0.05, 0.0, -1.0],
                [0.0, -0.05, 1.0]
            ]);
        assert_eq!(transform(abs, (0.0, 0.0)), (-1.0, 1.0));
        // The viewport size is 80 pixels, 40 points, therefore center is 20.
        assert_eq!(transform(abs, (20.0, 20.0)), (0.0, 0.0));
        assert_eq!(transform(abs, (40.0, 40.0)), (1.0, -1.0));
    }
}
