use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;
use std::thread;
use std::time::Duration;

mod app;
mod fractal;
mod ui;
mod utils;
// mod julia_set;  // We can comment this out since we're not using it anymore

use app::state::AppState;
use fractal::julia::measure_complexity;
use ui::terminal::{setup_terminal, cleanup_terminal, display_help};
use ui::renderer::render_fractal;

fn main() -> io::Result<()> {
    let screen_size = setup_terminal()?;
    
    let mut app_state = AppState::new();
    
    let initial_complexity = measure_complexity(
        &app_state.current_fractal, 
        screen_size.width, 
        screen_size.height
    );
    app_state.update_transition_time(initial_complexity);

    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => {
                        app_state.next_color_scheme();
                    }
                    KeyCode::Char('n') => {
                        let complexity = measure_complexity(
                            &app_state.next_fractal, 
                            screen_size.width, 
                            screen_size.height
                        );
                        app_state.start_new_transition(complexity);
                    }
                    KeyCode::Char('h') => {
                        app_state.toggle_help();
                        if !app_state.show_help {
                            render_fractal(
                                &app_state.current_fractal,
                                app_state.color_scheme, 
                                screen_size.width, 
                                screen_size.height
                            )?;
                        }
                    }
                    _ => {
                        if app_state.show_help {
                            app_state.show_help = false;
                            render_fractal(
                                &app_state.current_fractal,
                                app_state.color_scheme, 
                                screen_size.width, 
                                screen_size.height
                            )?;
                        }
                    }
                }
            }
        }

        if app_state.show_help {
            display_help()?;
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        let elapsed = app_state.transition_start.elapsed().as_secs_f64();

        if app_state.is_transition_complete(elapsed) {
            let complexity = measure_complexity(
                &app_state.next_fractal, 
                screen_size.width, 
                screen_size.height
            );
            app_state.start_new_transition(complexity);
        }

        let current_fractal = app_state.current_fractal_interpolated(elapsed);

        render_fractal(
            &current_fractal, 
            app_state.color_scheme,
            screen_size.width, 
            screen_size.height
        )?;

        thread::sleep(Duration::from_millis(50));
    }

    cleanup_terminal()?;
    Ok(())
}
