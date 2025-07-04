use colored::Colorize;
use num_complex::Complex;

use crate::julia_set::JuliaSet;

pub const MAX_ITERATIONS: u32 = 100;
pub const TRANSITION_TIME: f64 = 10.0;

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

pub fn get_color(iterations: u32, scheme: ColorScheme) -> String {
    if iterations == MAX_ITERATIONS {
        return " ".on_black().to_string();
    }

    // Scale iterations for color intensity
    let intensity = (iterations as f32 / MAX_ITERATIONS as f32 * 100.0) as u8;
    let char_selector = iterations % 4;

    // Select character based on depth
    let base_char = match char_selector {
        0 => "█",
        1 => "▓",
        2 => "▒",
        _ => "░",
    };

    match scheme {
        ColorScheme::Rainbow => match iterations % 16 {
            0 => "█".black().to_string(),
            1 => "█".blue().to_string(),
            2 => "█".cyan().to_string(),
            3 => "█".green().to_string(),
            4 => "█".magenta().to_string(),
            5 => "█".purple().to_string(),
            6 => "█".red().to_string(),
            7 => "█".yellow().to_string(),
            8 => "▓".blue().to_string(),
            9 => "▓".cyan().to_string(),
            10 => "▓".green().to_string(),
            11 => "▓".magenta().to_string(),
            12 => "▒".blue().to_string(),
            13 => "▒".cyan().to_string(),
            14 => "░".blue().to_string(),
            _ => "░".cyan().to_string(),
        },
        ColorScheme::Blues => {
            // Blues monochrome with higher contrast
            if intensity > 90 {
                base_char.bright_white().on_bright_blue().to_string()
            } else if intensity > 75 {
                base_char.white().on_blue().to_string()
            } else if intensity > 60 {
                base_char.bright_blue().to_string()
            } else if intensity > 45 {
                base_char.blue().to_string()
            } else if intensity > 30 {
                base_char.bright_black().on_bright_black().to_string()
            } else if intensity > 15 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
        ColorScheme::Greens => {
            // Greens monochrome with higher contrast
            if intensity > 6 {
                base_char.bright_white().on_bright_green().to_string()
            } else if intensity > 5 {
                base_char.white().on_green().to_string()
            } else if intensity > 4 {
                base_char.bright_green().to_string()
            } else if intensity > 3 {
                base_char.green().to_string()
            } else if intensity > 2 {
                " ".on_bright_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
        ColorScheme::Purples => {
            // Purples with higher contrast
            if intensity > 7 {
                base_char.bright_white().on_bright_magenta().to_string()
            } else if intensity > 6 {
                base_char.white().on_magenta().to_string()
            } else if intensity > 5 {
                base_char.bright_magenta().to_string()
            } else if intensity > 4 {
                base_char.magenta().to_string()
            } else if intensity > 3 {
                base_char.bright_magenta().on_bright_black().to_string()
            } else if intensity > 1 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        },
        ColorScheme::Grays => {
            // Updated grayscale to match Purple/Green threshold style
            if intensity > 7 {
                base_char.bright_white().on_white().to_string()
            } else if intensity > 6 {
                base_char.bright_white().to_string()
            } else if intensity > 5 {
                base_char.white().to_string()
            } else if intensity > 4 {
                base_char.bright_black().to_string()
            } else if intensity > 3 {
                base_char.black().on_bright_black().to_string()
            } else if intensity > 1 {
                base_char.black().on_black().to_string()
            } else {
                " ".on_black().to_string()
            }
        }
    }
}
