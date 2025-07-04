use num_complex::Complex;
use rand::prelude::*;

pub struct FractalParameters {
    pub c: Complex<f64>,
    pub zoom: f64,
    pub x_offset: f64,
    pub y_offset: f64,
}

impl FractalParameters {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        Self {
            c: Complex::new(
                rng.gen_range(-0.8..0.8),
                rng.gen_range(-0.8..0.8),
            ),
            zoom: rng.gen_range(0.5..1.5),
            x_offset: rng.gen_range(-0.5..0.5),
            y_offset: rng.gen_range(-0.5..0.5),
        }
    }
    
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            c: Complex::new(
                self.c.re * (1.0 - t) + other.c.re * t,
                self.c.im * (1.0 - t) + other.c.im * t,
            ),
            zoom: self.zoom * (1.0 - t) + other.zoom * t,
            x_offset: self.x_offset * (1.0 - t) + other.x_offset * t,
            y_offset: self.y_offset * (1.0 - t) + other.y_offset * t,
        }
    }
}
