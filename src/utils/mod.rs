pub mod constants {
    pub const MIN_TRANSITION_TIME: f64 = 5.0;
    pub const MAX_TRANSITION_TIME: f64 = 20.0;
}

// Re-export useful constants at the module level
pub use constants::*;
