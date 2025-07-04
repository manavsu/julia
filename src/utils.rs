use colored::Colorize;
use num_complex::Complex;

use crate::julia_set::JuliaSet;

pub const MAX_ITERATIONS: u32 = 100;
pub const TRANSITION_TIME: f64 = 10.0; // Keep original constant for compatibility
pub const MIN_TRANSITION_TIME: f64 = 5.0;  // Minimum transition time (fast)
pub const MAX_TRANSITION_TIME: f64 = 20.0; // Maximum transition time (slow)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Rainbow,
    Blues,
    Greens,
    Purples,
    Grays,
}

impl ColorScheme {
    pub fn next(&self) -> Self {
        match self {
            ColorScheme::Rainbow => ColorScheme::Blues,
            ColorScheme::Blues => ColorScheme::Greens,
            ColorScheme::Greens => ColorScheme::Purples,
            ColorScheme::Purples => ColorScheme::Grays,
            ColorScheme::Grays => ColorScheme::Rainbow,
        }
    }
}

pub fn calculate_julia(z: Complex<f64>, c: Complex<f64>) -> u32 {
    let mut z = z;
    let mut i = 0;

    while i < MAX_ITERATIONS && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }

    i
}

pub fn map_to_complex(x: u16, y: u16, width: u16, height: u16, julia: &JuliaSet) -> Complex<f64> {
    let aspect_ratio = height as f64 / width as f64;
    let real = (x as f64 / width as f64 - 0.5) * 3.0 / julia.zoom + julia.x_offset;
    let imag = (y as f64 / height as f64 - 0.5) * 3.0 * aspect_ratio / julia.zoom + julia.y_offset;
    Complex::new(real, imag)
}

// Sample the Julia set to determine its complexity/interestingness
pub fn calculate_transition_time(julia: &JuliaSet, width: u16, height: u16) -> f64 {
    // Take a sample of points to analyze complexity
    let sample_size = 100;
    let mut iteration_counts = Vec::with_capacity(sample_size);
    
    // Sample evenly across the screen
    for i in 0..sample_size {
        let x = (i % 10) as u16 * (width / 10).max(1);
        let y = (i / 10) as u16 * (height / 10).max(1);
        
        let z = map_to_complex(x, y, width, height, julia);
        let iterations = calculate_julia(z, julia.c);
        iteration_counts.push(iterations);
    }
    
    // Calculate standard deviation to measure complexity
    let mean = iteration_counts.iter().sum::<u32>() as f64 / sample_size as f64;
    
    // Handle potential empty sample
    if iteration_counts.is_empty() {
        return TRANSITION_TIME; // Default to constant
    }
    
    let variance = iteration_counts.iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / sample_size as f64;
    
    // Protect against NaN
    if variance.is_nan() || variance <= 0.0 {
        return TRANSITION_TIME; // Default to constant
    }
    
    let std_dev = variance.sqrt();
    
    // Normalize the standard deviation to a 0.0-1.0 range
    // Higher std_dev means more interesting patterns
    let normalized_complexity = (std_dev / (MAX_ITERATIONS as f64 / 2.0)).min(1.0);
    
    // Calculate transition time - more complex = slower transitions
    let transition_time = MIN_TRANSITION_TIME + normalized_complexity * (MAX_TRANSITION_TIME - MIN_TRANSITION_TIME);
    
    // Print info for debugging
    println!("Complexity: {:.2}, Time: {:.2}s", normalized_complexity, transition_time);
    
    transition_time
}

pub fn get_color(iterations: u32, scheme: ColorScheme) -> String {
    if iterations == MAX_ITERATIONS {
        return " ".on_black().to_string();
    }

    // Use iterations directly for character selection
    let char_selector = iterations % 4;

    // Select character based on iterations
    let base_char = match char_selector {
        0 => "█",
        1 => "▓",
        2 => "▒",
        _ => "░",
    };

    match scheme {
        ColorScheme::Rainbow => match iterations % 16 {
            0 => base_char.black().to_string(),
            1 => base_char.blue().to_string(),
            2 => base_char.cyan().to_string(),
            3 => base_char.green().to_string(),
            4 => base_char.magenta().to_string(),
            5 => base_char.purple().to_string(),
            6 => base_char.red().to_string(),
            7 => base_char.yellow().to_string(),
            8 => base_char.blue().to_string(),
            9 => base_char.cyan().to_string(),
            10 => base_char.green().to_string(),
            11 => base_char.magenta().to_string(),
            12 => base_char.blue().to_string(),
            13 => base_char.cyan().to_string(),
            14 => base_char.blue().to_string(),
            _ => base_char.cyan().to_string(),
        },
        ColorScheme::Blues => {
            // Single-digit iteration thresholds
            if iterations > 9 {
                base_char.bright_white().on_bright_blue().to_string()
            } else if iterations > 7 {
                base_char.white().on_blue().to_string()
            } else if iterations > 5 {
                base_char.bright_blue().to_string()
            } else if iterations > 3 {
                base_char.blue().to_string()
            } else if iterations > 2 {
                base_char.bright_black().on_bright_black().to_string()
            } else if iterations > 1 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
        ColorScheme::Greens => {
            // Single-digit iteration thresholds
            if iterations > 9 {
                base_char.bright_white().on_bright_green().to_string()
            } else if iterations > 7 {
                base_char.white().on_green().to_string()
            } else if iterations > 5 {
                base_char.bright_green().to_string()
            } else if iterations > 3 {
                base_char.green().to_string()
            } else if iterations > 2 {
                " ".on_bright_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
        ColorScheme::Purples => {
            // Single-digit iteration thresholds
            if iterations > 9 {
                base_char.bright_white().on_bright_magenta().to_string()
            } else if iterations > 7 {
                base_char.white().on_magenta().to_string()
            } else if iterations > 5 {
                base_char.bright_magenta().to_string()
            } else if iterations > 3 {
                base_char.magenta().to_string()
            } else if iterations > 2 {
                base_char.bright_magenta().on_bright_black().to_string()
            } else if iterations > 1 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        },
        ColorScheme::Grays => {
            // Single-digit iteration thresholds
            if iterations > 9 {
                base_char.bright_white().on_white().to_string()
            } else if iterations > 7 {
                base_char.bright_white().to_string()
            } else if iterations > 5 {
                base_char.white().to_string()
            } else if iterations > 3 {
                base_char.bright_black().to_string()
            } else if iterations > 2 {
                base_char.black().on_bright_black().to_string()
            } else if iterations > 1 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
    }
}
