use num_complex::Complex;
use super::parameters::FractalParameters;
use crate::julia_set::JuliaSet;

pub const MAX_ITERATIONS: u32 = 100;

pub fn calculate_iterations(z: Complex<f64>, c: Complex<f64>) -> u32 {
    let mut z = z;
    let mut i = 0;

    while i < MAX_ITERATIONS && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }

    i
}

pub fn map_point_to_complex(x: u16, y: u16, width: u16, height: u16, params: &FractalParameters) -> Complex<f64> {
    let aspect_ratio = height as f64 / width as f64;
    let real = (x as f64 / width as f64 - 0.5) * 3.0 / params.zoom + params.x_offset;
    let imag = (y as f64 / height as f64 - 0.5) * 3.0 * aspect_ratio / params.zoom + params.y_offset;
    Complex::new(real, imag)
}

// Compatibility function for the old JuliaSet type
pub fn map_julia_to_complex(x: u16, y: u16, width: u16, height: u16, julia: &JuliaSet) -> Complex<f64> {
    let aspect_ratio = height as f64 / width as f64;
    let real = (x as f64 / width as f64 - 0.5) * 3.0 / julia.zoom + julia.x_offset;
    let imag = (y as f64 / height as f64 - 0.5) * 3.0 * aspect_ratio / julia.zoom + julia.y_offset;
    Complex::new(real, imag)
}

pub fn measure_complexity(params: &FractalParameters, width: u16, height: u16) -> f64 {
    let sample_size = 100;
    let mut iteration_counts = Vec::with_capacity(sample_size);
    
    for i in 0..sample_size {
        let x = (i % 10) as u16 * (width / 10).max(1);
        let y = (i / 10) as u16 * (height / 10).max(1);
        
        let z = map_point_to_complex(x, y, width, height, params);
        let iterations = calculate_iterations(z, params.c);
        iteration_counts.push(iterations);
    }
    
    let mean = iteration_counts.iter().sum::<u32>() as f64 / sample_size as f64;
    
    if iteration_counts.is_empty() {
        return 0.5;
    }
    
    let variance = iteration_counts.iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / sample_size as f64;
    
    if variance.is_nan() || variance <= 0.0 {
        return 0.5;
    }
    
    let std_dev = variance.sqrt();
    
    (std_dev / (MAX_ITERATIONS as f64 / 2.0)).min(1.0)
}
