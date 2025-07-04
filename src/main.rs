use colored::Colorize;
use crossterm::{
    cursor, execute, terminal,
    event::{self, Event, KeyCode, KeyEvent},
};
use num_complex::Complex;
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    thread, time::{Duration, Instant},
};

const MAX_ITERATIONS: u32 = 100;
const TRANSITION_TIME: f64 = 10.0; // Time in seconds to transition between Julia sets

/// Represents a Julia set with its parameters
struct JuliaSet {
    c: Complex<f64>,
    zoom: f64,
    x_offset: f64,
    y_offset: f64,
}

impl JuliaSet {
    fn new(c_real: f64, c_imag: f64, zoom: f64, x_offset: f64, y_offset: f64) -> Self {
        Self {
            c: Complex::new(c_real, c_imag),
            zoom,
            x_offset,
            y_offset,
        }
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate parameters that tend to create interesting fractals
        let r = rng.gen_range(0.3..0.8);
        let theta = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let c_real = r * theta.cos();
        let c_imag = r * theta.sin();
        
        let zoom = rng.gen_range(0.5..2.0);
        let x_offset = rng.gen_range(-0.5..0.5);
        let y_offset = rng.gen_range(-0.5..0.5);
        
        Self::new(c_real, c_imag, zoom, x_offset, y_offset)
    }

    /// Linearly interpolate between two Julia sets
    fn lerp(&self, other: &JuliaSet, t: f64) -> JuliaSet {
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

/// Calculate how many iterations it takes for a point to escape in the Julia set
fn calculate_julia(z: Complex<f64>, c: Complex<f64>) -> u32 {
    let mut z = z;
    let mut i = 0;
    
    while i < MAX_ITERATIONS && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }
    
    i
}

/// Map screen coordinates to the complex plane
fn map_to_complex(x: u16, y: u16, width: u16, height: u16, julia: &JuliaSet) -> Complex<f64> {
    let aspect_ratio = height as f64 / width as f64;
    let real = (x as f64 / width as f64 - 0.5) * 3.0 / julia.zoom + julia.x_offset;
    let imag = (y as f64 / height as f64 - 0.5) * 3.0 * aspect_ratio / julia.zoom + julia.y_offset;
    Complex::new(real, imag)
}

/// Choose a color based on the number of iterations
fn get_color(iterations: u32) -> String {
    if iterations == MAX_ITERATIONS {
        return " ".on_black().to_string();
    }
    
    // Map the iterations to a color
    let hue = (iterations as f64 / MAX_ITERATIONS as f64 * 360.0) as u8;
    
    // Simple color palette based on iterations
    match iterations % 16 {
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
    }
}

/// Render a Julia set to the terminal
fn render_julia_set(julia: &JuliaSet) -> io::Result<()> {
    let mut stdout = stdout();
    
    // Get terminal size
    let (width, height) = terminal::size()?;
    
    // Move cursor to top-left
    execute!(stdout, cursor::MoveTo(0, 0))?;
    
    // For each character position in the terminal
    for y in 0..height {
        for x in 0..width {
            let z = map_to_complex(x, y, width, height, julia);
            let iterations = calculate_julia(z, julia.c);
            let color_char = get_color(iterations);
            
            write!(stdout, "{}", color_char)?;
        }
    }
    
    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
    )?;
    
    // Set up Julia sets for animation
    let mut current_julia = JuliaSet::random();
    let mut next_julia = JuliaSet::random();
    let mut transition_start = Instant::now();
    
    // Main loop
    loop {
        // Check for key press to exit
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event::read()? {
                break;
            }
        }
        
        // Calculate transition progress
        let elapsed = transition_start.elapsed().as_secs_f64();
        let transition_progress = (elapsed / TRANSITION_TIME).min(1.0);
        
        // If transition is complete, start a new one
        if transition_progress >= 1.0 {
            current_julia = next_julia;
            next_julia = JuliaSet::random();
            transition_start = Instant::now();
        }
        
        // Interpolate between current and next Julia sets
        let interpolated_julia = current_julia.lerp(&next_julia, transition_progress);
        
        // Render the interpolated Julia set
        render_julia_set(&interpolated_julia)?;
        
        // Short sleep to avoid maxing out CPU
        thread::sleep(Duration::from_millis(50));
    }
    
    // Clean up terminal
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
    )?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}
