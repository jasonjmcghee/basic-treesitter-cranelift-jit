use std::{io, thread};
use std::io::Write;
use std::time::{Duration, Instant};
use crossterm::cursor::{MoveTo};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::style::{SetBackgroundColor, SetForegroundColor, Color, ResetColor};
use crossterm::{event, execute};
use miette::{IntoDiagnostic, Result as MietteResult};
use rand::prelude::IndexedRandom;
use rand::Rng;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Query, QueryCursor};
use crate::language::Calculator;

const SHOULD_HIGHLIGHT: bool = true;

#[derive(Debug)]
enum InputAction {
    InsertChar(char),
    Backspace,
    Delete,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorHome,
    MoveCursorEnd,
    SetContent(String),  // New action for setting entire content at once
}
struct InputState {
    content: String,
    cursor_position: usize,
    highlight_query: Query,
}

impl InputState {
    fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            highlight_query: Query::new(&tree_sitter_calculator::LANGUAGE.into(), include_str!("../tree-sitter-calculator/queries/highlights.scm"))
                .expect("Failed to load highlights query"),
        }
    }

    fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    fn backspace(&mut self) -> bool {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
            true
        } else {
            false
        }
    }

    fn delete(&mut self) -> bool {
        if self.cursor_position < self.content.len() {
            self.content.remove(self.cursor_position);
            true
        } else {
            false
        }
    }

    fn move_cursor_left(&mut self) -> bool {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            true
        } else {
            false
        }
    }

    fn move_cursor_right(&mut self) -> bool {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
            true
        } else {
            false
        }
    }

    fn move_cursor_home(&mut self) -> bool {
        if self.cursor_position != 0 {
            self.cursor_position = 0;
            true
        } else {
            false
        }
    }

    fn move_cursor_end(&mut self) -> bool {
        if self.cursor_position != self.content.len() {
            self.cursor_position = self.content.len();
            true
        } else {
            false
        }
    }

    fn handle_action(&mut self, action: InputAction) -> bool {
        match action {
            InputAction::InsertChar(c) => {
                self.insert_char(c);
                true
            }
            InputAction::Backspace => self.backspace(),
            InputAction::Delete => self.delete(),
            InputAction::MoveCursorLeft => self.move_cursor_left(),
            InputAction::MoveCursorRight => self.move_cursor_right(),
            InputAction::MoveCursorHome => self.move_cursor_home(),
            InputAction::MoveCursorEnd => self.move_cursor_end(),
            InputAction::SetContent(content) => {
                self.content = content;
                self.cursor_position = self.content.len();
                true
            }
        }
    }

    fn draw_input_line(&self, stdout: &mut io::Stdout, calculator: &mut Calculator) -> MietteResult<()> {
        // Clear the entire screen instead of just the current line
        execute!(stdout, Clear(ClearType::CurrentLine)).into_diagnostic()?;
        execute!(stdout, MoveTo(0, 0)).into_diagnostic()?;

        if SHOULD_HIGHLIGHT {
            // Parse the current content
            let tree = calculator.parser.parse(&self.content, None)
                .expect("Failed to parse");

            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&self.highlight_query, tree.root_node(), self.content.as_bytes());

            let mut captures: Vec<tree_sitter::QueryCapture<'_>> = Vec::new();
            while let Some(m) = matches.next() {
                captures.extend(m.captures);
            }
            // Sort by start byte, with longer ranges first when start bytes are equal
            captures.sort_by_key(|c| (
                c.node.start_byte(),
                std::cmp::Reverse(c.node.end_byte() - c.node.start_byte())
            ));

            // Remove captures that overlap with previous ones
            captures.dedup_by(|a, b| {
                let a_range = a.node.start_byte()..a.node.end_byte();
                let b_range = b.node.start_byte()..b.node.end_byte();
                a_range.contains(&b.node.start_byte()) || b_range.contains(&a.node.start_byte())
            });

            let mut last_byte = 0;
            let content_bytes = self.content.as_bytes();

            // Apply colors based on node types
            for capture in captures {
                let start_byte = capture.node.start_byte();
                let end_byte = capture.node.end_byte();

                // Write any text before this capture with default color
                if start_byte > last_byte {
                    let segment = std::str::from_utf8(&content_bytes[last_byte..start_byte])
                        .expect("Invalid UTF-8");
                    write!(stdout, "{}", segment).into_diagnostic()?;
                }

                // Set color based on capture name
                match self.highlight_query.capture_names()[capture.index as usize].as_str() {
                    "operator" => execute!(stdout, SetForegroundColor(Color::Yellow)).into_diagnostic()?,
                    "number" => execute!(stdout, SetForegroundColor(Color::Green)).into_diagnostic()?,
                    "float" => execute!(stdout, SetForegroundColor(Color::Cyan)).into_diagnostic()?,
                    "error" => execute!(stdout, SetForegroundColor(Color::Red)).into_diagnostic()?,
                    _ => execute!(stdout, ResetColor).into_diagnostic()?,
                }

                // Write the highlighted segment
                let segment = std::str::from_utf8(&content_bytes[start_byte..end_byte])
                    .expect("Invalid UTF-8");
                write!(stdout, "{}", segment).into_diagnostic()?;
                execute!(stdout, ResetColor).into_diagnostic()?;

                last_byte = end_byte;
            }

            // Write any remaining text
            if last_byte < self.content.len() {
                let segment = std::str::from_utf8(&content_bytes[last_byte..])
                    .expect("Invalid UTF-8");
                write!(stdout, "{}", segment).into_diagnostic()?;
            }
        } else {
            // No highlighting
            write!(stdout, "{}", &self.content).into_diagnostic()?;
        }

        // Draw the cursor
        execute!(
            stdout,
            MoveTo(self.cursor_position as u16, 0),
            SetBackgroundColor(Color::White),
        ).into_diagnostic()?;
        write!(stdout, " ").into_diagnostic()?;
        execute!(stdout, ResetColor).into_diagnostic()?;

        stdout.flush().into_diagnostic()
    }
}

pub fn run_repl() -> MietteResult<()> {
    let mut calculator = Calculator::new()?;
    let mut input_state = InputState::new();
    let mut last_input = String::new();
    let mut stdout = io::stdout();

    // Configure miette for terminal output
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(false)
                .unicode(true)
                .width(80)
                .force_graphical(true)
                .build(),
        )
    }))?;

    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    enable_raw_mode().into_diagnostic()?;

    let result = (|| -> MietteResult<()> {
        // Initial draw
        input_state.draw_input_line(&mut stdout, &mut calculator)?;

        loop {
            if let Ok(true) = event::poll(Duration::from_millis(10)) {
                match event::read().into_diagnostic()? {
                    Event::Key(KeyEvent {
                                   code: KeyCode::Char('c'),
                                   modifiers: KeyModifiers::CONTROL,
                                   ..
                               }) => break,

                    Event::Key(KeyEvent { code, modifiers, .. }) => {
                        let action = match (code, modifiers) {
                            (KeyCode::Char(c), KeyModifiers::NONE) => Some(InputAction::InsertChar(c)),
                            (KeyCode::Backspace, _) => Some(InputAction::Backspace),
                            (KeyCode::Delete, _) => Some(InputAction::Delete),
                            (KeyCode::Left, _) => Some(InputAction::MoveCursorLeft),
                            (KeyCode::Right, _) => Some(InputAction::MoveCursorRight),
                            (KeyCode::Home, _) => Some(InputAction::MoveCursorHome),
                            (KeyCode::End, _) => Some(InputAction::MoveCursorEnd),
                            _ => None,
                        };

                        let input_changed = if let Some(action) = action {
                            input_state.handle_action(action)
                        } else {
                            false
                        };

                        if input_changed {
                            let common_prefix = input_state
                                .content
                                .chars()
                                .zip(last_input.chars())
                                .take_while(|(a, b)| a == b)
                                .count();

                            execute!(stdout, Clear(ClearType::All)).into_diagnostic()?;

                            // Draw input line with cursor
                            input_state.draw_input_line(&mut stdout, &mut calculator)?;

                            match calculator.update_input(
                                &input_state.content,
                                common_prefix,
                                last_input.len(),
                                input_state.content.len(),
                            ) {
                                Ok(value) => {
                                    execute!(stdout, MoveTo(0, 1)).into_diagnostic()?;
                                    writeln!(stdout, "= {:?}", value).into_diagnostic()?;
                                    // Return cursor to input line
                                    execute!(stdout, MoveTo(input_state.cursor_position as u16, 0)).into_diagnostic()?;
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
                                    // Return cursor to input line
                                    execute!(stdout, MoveTo(input_state.cursor_position as u16, 0)).into_diagnostic()?;
                                }
                            }

                            stdout.flush().into_diagnostic()?;
                            last_input = input_state.content.clone();
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode().into_diagnostic()?;
    execute!(stdout, LeaveAlternateScreen).into_diagnostic()?;

    result
}


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
        let sleep_duration = Duration::from_micros((1_000_000.0 / updates_per_second as f64) as u64);
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
                                  }) = event::read().into_diagnostic()? {
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
pub fn test() -> MietteResult<()> {
    if let Err(e) = run_automated_test(1000, 30) {
        eprintln!("Error during performance test: {:?}", e);
    }
    Ok(())
}