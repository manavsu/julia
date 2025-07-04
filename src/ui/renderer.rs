use std::io::{self, Write, stdout};
use crossterm::{cursor, execute};

use crate::fractal::{FractalParameters, calculate_iterations, map_point_to_complex};
use crate::ui::ColorScheme;
use crate::ui::colors::get_color;

pub fn render_fractal(params: &FractalParameters, color_scheme: ColorScheme, width: u16, height: u16) -> io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, cursor::MoveTo(0, 0))?;

    for y in 0..height {
        for x in 0..width {
            let z = map_point_to_complex(x, y, width, height, params);
            let iterations = calculate_iterations(z, params.c);
            let color_char = get_color(iterations, color_scheme);

            write!(stdout, "{}", color_char)?;
        }
    }

    stdout.flush()?;
    Ok(())
}
