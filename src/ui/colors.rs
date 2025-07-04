use colored::Colorize;
use crate::fractal::MAX_ITERATIONS;

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

pub fn get_color(iterations: u32, scheme: ColorScheme) -> String {
    if iterations == MAX_ITERATIONS {
        return " ".on_black().to_string();
    }

    let char_selector = iterations % 4;

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
        ColorScheme::Blues => color_gradient(iterations, base_char, "blue"),
        ColorScheme::Greens => color_gradient(iterations, base_char, "green"),
        ColorScheme::Purples => color_gradient(iterations, base_char, "magenta"),
        ColorScheme::Grays => color_gradient(iterations, base_char, "gray"),
    }
}

fn color_gradient(iterations: u32, base_char: &str, color_type: &str) -> String {
    match color_type {
        "blue" => {
            if iterations > 8 {
                base_char.bright_white().on_bright_blue().to_string()
            } else if iterations > 6 {
                base_char.white().on_blue().to_string()
            } else if iterations > 4 {
                base_char.bright_blue().to_string()
            } else if iterations > 2 {
                base_char.blue().to_string()
            } else if iterations > 1 {
                base_char.bright_black().on_bright_black().to_string()
            } else {
                base_char.black().on_black().to_string()
            }
        },
        "green" => {
            if iterations > 8 {
                base_char.bright_white().on_bright_green().to_string()
            } else if iterations > 6 {
                base_char.white().on_green().to_string()
            } else if iterations > 4 {
                base_char.bright_green().to_string()
            } else if iterations > 2 {
                base_char.green().to_string()
            } else if iterations > 1 {
                base_char.bright_black().on_bright_black().to_string()
            } else {
                base_char.black().on_black().to_string()
            }
        },
        "magenta" => {
            if iterations > 8 {
                base_char.bright_white().on_bright_magenta().to_string()
            } else if iterations > 6 {
                base_char.white().on_magenta().to_string()
            } else if iterations > 4 {
                base_char.bright_magenta().to_string()
            } else if iterations > 2 {
                base_char.magenta().to_string()
            } else if iterations > 1 {
                base_char.bright_black().on_bright_black().to_string()
            } else {
                base_char.black().on_black().to_string()
            }
        },
        "gray" | _ => {
            if iterations > 8 {
                base_char.bright_white().on_white().to_string()
            } else if iterations > 6 {
                base_char.bright_white().to_string()
            } else if iterations > 4 {
                base_char.white().to_string()
            } else if iterations > 2 {
                base_char.bright_black().to_string()
            } else if iterations > 1 {
                base_char.black().on_bright_black().to_string()
            } else {
                base_char.black().on_black().to_string()
            }
        }
    }
}
