use num_complex::Complex;
use rand::Rng;

pub struct JuliaSet {
    pub c: Complex<f64>,
    pub zoom: f64,
    pub x_offset: f64,
    pub y_offset: f64,
}

impl JuliaSet {
    pub fn new(c_real: f64, c_imag: f64, zoom: f64, x_offset: f64, y_offset: f64) -> Self {
        Self {
            c: Complex::new(c_real, c_imag),
            zoom,
            x_offset,
            y_offset,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(0.3..0.8);
        let theta = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let c_real = r * theta.cos();
        let c_imag = r * theta.sin();

        let zoom = rng.gen_range(0.5..2.0);
        let x_offset = rng.gen_range(-0.5..0.5);
        let y_offset = rng.gen_range(-0.5..0.5);

        Self::new(c_real, c_imag, zoom, x_offset, y_offset)
    }

    pub fn lerp(&self, other: &JuliaSet, t: f64) -> JuliaSet {
        let t = t.clamp(0.0, 1.0);
        JuliaSet {
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

