use std::time::Instant;

use crate::fractal::FractalParameters;
use crate::ui::ColorScheme;
use crate::utils::constants::{MIN_TRANSITION_TIME, MAX_TRANSITION_TIME};

pub struct AppState {
    pub current_fractal: FractalParameters,
    pub next_fractal: FractalParameters,
    pub transition_start: Instant,
    pub transition_time: f64,
    pub color_scheme: ColorScheme,
    pub show_help: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_fractal: FractalParameters::random(),
            next_fractal: FractalParameters::random(),
            transition_start: Instant::now(),
            transition_time: 10.0,
            color_scheme: ColorScheme::Rainbow,
            show_help: false,
        }
    }

    pub fn next_color_scheme(&mut self) {
        self.color_scheme = self.color_scheme.next();
    }

    pub fn start_new_transition(&mut self, complexity: f64) {
        self.current_fractal = self.next_fractal;
        self.next_fractal = FractalParameters::random();
        self.transition_start = Instant::now();
        self.update_transition_time(complexity);
    }

    pub fn update_transition_time(&mut self, complexity: f64) {
        self.transition_time = MIN_TRANSITION_TIME + complexity * (MAX_TRANSITION_TIME - MIN_TRANSITION_TIME);
        println!("Complexity: {:.2}, Time: {:.2}s", complexity, self.transition_time);
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn current_fractal_interpolated(&self, elapsed_seconds: f64) -> FractalParameters {
        let progress = (elapsed_seconds / self.transition_time).min(1.0);
        self.current_fractal.lerp(&self.next_fractal, progress)
    }

    pub fn is_transition_complete(&self, elapsed_seconds: f64) -> bool {
        elapsed_seconds >= self.transition_time
    }
}
