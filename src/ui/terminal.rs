use crossterm::{cursor, execute, terminal};
use std::io::{self, Write, stdout};

pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
}

pub fn setup_terminal() -> io::Result<ScreenSize> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;
    
    let (width, height) = terminal::size()?;
    Ok(ScreenSize { width, height })
}

pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show,)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn display_help() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(0, 0))?;
    println!("Julia Set Fractal Screensaver");
    println!("---------------------------");
    println!("Commands:");
    println!("  q: Quit");
    println!("  c: Change color scheme");
    println!("  n: New random Julia set");
    println!("  h: Show/hide this help");
    println!("Press any key to continue...");
    stdout.flush()?;
    Ok(())
}
