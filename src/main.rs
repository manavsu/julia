use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::{self, Write, stdout};
use std::thread;
use std::time::{Duration, Instant};

mod julia_set;
mod renderer;
mod utils;

use julia_set::JuliaSet;
use renderer::{cleanup_terminal, render_julia_set, setup_terminal};
use utils::{ColorScheme, calculate_transition_time};

fn display_help() -> io::Result<()> {
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

fn main() -> io::Result<()> {
    // Set up terminal
    let (width, height) = setup_terminal()?;

    // Set up Julia sets for animation
    let mut current_julia = JuliaSet::random();
    let mut next_julia = JuliaSet::random();
    let mut transition_start = Instant::now();
    let mut color_scheme = ColorScheme::Rainbow;
    let mut show_help = false;
    
    // Calculate initial transition time based on complexity
    let mut current_transition_time = calculate_transition_time(&current_julia, width, height);

    // Main loop
    loop {
        // Check for key press
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => {
                        color_scheme = color_scheme.next();
                    }
                    KeyCode::Char('n') => {
                        current_julia = next_julia;
                        next_julia = JuliaSet::random();
                        transition_start = Instant::now();
                        // Recalculate transition time for new julia set
                        current_transition_time = calculate_transition_time(&current_julia, width, height);
                    }
                    KeyCode::Char('h') => {
                        show_help = !show_help;
                        if !show_help {
                            // Force a full redraw when hiding help
                            render_julia_set(&current_julia, color_scheme)?
                        }
                    }
                    _ => {
                        if show_help {
                            show_help = false;
                            // Force a full redraw when hiding help
                            render_julia_set(&current_julia, color_scheme)?
                        }
                    }
                }
            }
        }

        if show_help {
            display_help()?;
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        // Calculate transition progress using dynamic transition time
        let elapsed = transition_start.elapsed().as_secs_f64();
        let transition_progress = (elapsed / current_transition_time).min(1.0);

        // If transition is complete, start a new one
        if transition_progress >= 1.0 {
            current_julia = next_julia;
            next_julia = JuliaSet::random();
            transition_start = Instant::now();
            
            // Recalculate transition time based on complexity of new Julia set
            current_transition_time = calculate_transition_time(&current_julia, width, height);
        }

        // Interpolate between current and next Julia sets
        let interpolated_julia = current_julia.lerp(&next_julia, transition_progress);

        // Render the interpolated Julia set
        render_julia_set(&interpolated_julia, color_scheme)?;

        // Short sleep to avoid maxing out CPU
        thread::sleep(Duration::from_millis(50));
    }

    // Clean up terminal
    cleanup_terminal()?;

    Ok(())
}
