use crossterm::{cursor, execute, terminal};
use std::io::{self, Write, stdout};

use crate::julia_set::JuliaSet;
use crate::utils::{ColorScheme, calculate_julia, get_color, map_to_complex};

pub fn render_julia_set(julia: &JuliaSet, color_scheme: ColorScheme) -> io::Result<()> {
    let mut stdout = stdout();

    let (width, height) = terminal::size()?;

    execute!(stdout, cursor::MoveTo(0, 0))?;

    for y in 0..height {
        for x in 0..width {
            let z = map_to_complex(x, y, width, height, julia);
            let iterations = calculate_julia(z, julia.c);
            let color_char = get_color(iterations, color_scheme);

            write!(stdout, "{}", color_char)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

pub fn setup_terminal() -> io::Result<(u16, u16)> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;
    
    // Return terminal size
    let (width, height) = terminal::size()?;
    Ok((width, height))
}

pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show,)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
