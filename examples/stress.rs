use adder_treesitter_cranelift::language::Calculator;
use adder_treesitter_cranelift::repl::{InputAction, InputState};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use miette::{IntoDiagnostic, Result as MietteResult};
use rand::prelude::IndexedRandom;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{io, thread};

struct ExpressionConfig {
    max_depth: u32,
    max_terms: u32,
    allow_floats: bool,
    allow_parens: bool,
    allow_negatives: bool,
}

impl Default for ExpressionConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_terms: 5,
            allow_floats: true,
            allow_parens: false,
            allow_negatives: true,
        }
    }
}

fn generate_complex_expression(config: &ExpressionConfig) -> String {
    let mut rng = rand::thread_rng();
    generate_expression(config.max_depth, config, &mut rng)
}

fn generate_expression(depth: u32, config: &ExpressionConfig, rng: &mut impl rand::Rng) -> String {
    if depth == 0 || rng.gen_bool(0.3) {
        // Generate terminal (number)
        generate_number(config, rng)
    } else {
        // Decide between binary operation or parenthesized expression
        if config.allow_parens && depth > 1 && rng.gen_bool(0.4) {
            // Parenthesized expression
            format!("({})", generate_expression(depth - 1, config, rng))
        } else {
            // Binary operation
            let num_terms = rng.gen_range(2..=config.max_terms.min(5));
            let mut expr = generate_expression(depth - 1, config, rng);

            for _ in 1..num_terms {
                let operator = *["+", "-", "*", "/"].choose(rng).unwrap();
                expr.push_str(operator);
                expr.push_str(&generate_expression(depth - 1, config, rng));
            }
            expr
        }
    }
}

fn generate_number(config: &ExpressionConfig, rng: &mut impl rand::Rng) -> String {
    let is_float = config.allow_floats && rng.gen_bool(0.3);
    let is_negative = config.allow_negatives && rng.gen_bool(0.3);

    let num = if is_float {
        format!("{:.2}", rng.gen_range(0.0..1000.0))
    } else {
        rng.gen_range(0..1000).to_string()
    };

    if is_negative {
        format!("-{}", num)
    } else {
        num
    }
}

// Modified automated test function to use complex expressions
pub fn run_automated_test(updates_per_second: u32, duration_secs: u64) -> MietteResult<()> {
    let mut calculator = Calculator::new()?;
    let mut input_state = InputState::new();
    let mut stdout = io::stdout();

    let config = ExpressionConfig {
        max_depth: 3,
        max_terms: 4,
        allow_floats: true,
        allow_parens: false,
        allow_negatives: true,
    };

    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    enable_raw_mode().into_diagnostic()?;

    let result = (|| -> MietteResult<()> {
        let start_time = Instant::now();
        let sleep_duration =
            Duration::from_micros((1_000_000.0 / updates_per_second as f64) as u64);
        let mut frame_count = 0;
        let mut last_fps_update = Instant::now();
        let mut current_fps = 0.0;

        while start_time.elapsed().as_secs() < duration_secs {
            // Generate a complex expression
            let expr = generate_complex_expression(&config);
            input_state.handle_action(InputAction::SetContent(expr));

            execute!(stdout, Clear(ClearType::All)).into_diagnostic()?;

            // Draw input line with cursor
            input_state.draw_input_line(&mut stdout, &mut calculator)?;

            // Process calculation
            match calculator.update_input(&input_state.content, 0, 0, input_state.content.len()) {
                Ok(value) => {
                    execute!(stdout, MoveTo(0, 1)).into_diagnostic()?;
                    writeln!(stdout, "= {:?}", value).into_diagnostic()?;
                }
                Err(error) => {
                    let mut error_buf = Vec::new();
                    writeln!(error_buf, "{:?}", error).into_diagnostic()?;
                    let error_str = String::from_utf8_lossy(&error_buf);

                    let mut current_row = 1;
                    for line in error_str.lines() {
                        if !line.is_empty() {
                            execute!(stdout, MoveTo(0, current_row)).into_diagnostic()?;
                            writeln!(stdout, "{}", line).into_diagnostic()?;
                            current_row += 1;
                        }
                    }
                }
            }

            // Update FPS counter
            frame_count += 1;
            let elapsed = last_fps_update.elapsed().as_secs_f64();
            if elapsed >= 1.0 {
                current_fps = frame_count as f64 / elapsed;
                frame_count = 0;
                last_fps_update = Instant::now();
            }

            // Move below any potential error messages and display stats
            execute!(stdout, MoveTo(0, 10)).into_diagnostic()?;
            writeln!(stdout, "FPS: {:.2}", current_fps).into_diagnostic()?;

            // Return cursor to input line position
            execute!(stdout, MoveTo(input_state.cursor_position as u16, 0)).into_diagnostic()?;

            stdout.flush().into_diagnostic()?;
            thread::sleep(sleep_duration);

            if event::poll(Duration::from_millis(0)).into_diagnostic()? {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) = event::read().into_diagnostic()?
                {
                    break;
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode().into_diagnostic()?;
    execute!(stdout, LeaveAlternateScreen).into_diagnostic()?;

    result
}

// Example usage in main.rs:
pub fn main() -> MietteResult<()> {
    if let Err(e) = run_automated_test(1000, 30) {
        eprintln!("Error during performance test: {:?}", e);
    }
    Ok(())
}
