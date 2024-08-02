use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use element::TuiElement;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, stdout, Stdout, Write};

pub mod button;
pub mod cursor;
pub mod element;
pub mod input;
pub mod selection;
pub mod text;

use cursor::Cursor;

#[macro_export]
macro_rules! tui_debug {
    ($stdout:expr, $($arg:expr),*) => {{
        use std::io::Write;
        let term_size = crossterm::terminal::size().unwrap();
        let (x, y) = term_size;
        queue!($stdout, MoveTo(0, y - 1), Print("DEBUG: ")).unwrap();
        $(queue!($stdout, Print($arg)).unwrap();)*
        $stdout.flush().unwrap();
    }};
}

#[macro_export]
macro_rules! tui_print_at {
    ($s:expr, $x:expr, $y:expr, $($arg:expr),*) => {{
        queue!($s, MoveTo($x, $y)).unwrap();
        $(queue!($s, Print($arg)).unwrap();)*
    }};
}

pub struct Tui {
    // ...
    // The terminal size
    pub size: (u16, u16),
    // The terminal cursor position
    cursor: Cursor,
    // The elements to draw
    pub elements: Vec<Box<dyn TuiElement>>,
    stdout: io::Stdout,
}

impl Tui {
    /// Create a new Tui instance
    /// This will initialize the terminal size and cursor position
    /// and create an empty elements vector
    ///
    /// Also, it will enable raw mode and enter an alternate screen, and disable it when the Tui
    /// instance is dropped.
    ///
    ///
    ///
    /// # Returns
    /// A new Tui instance
    pub fn new() -> Self {
        let (x, y) = crossterm::terminal::size().unwrap();
        enable_raw_mode().unwrap();
        queue!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        Self {
            size: (x, y),
            cursor: Cursor::default(),
            elements: Vec::new(),
            stdout: stdout(),
        }
    }
    // Draw should not alter the state of the Tui
    pub fn draw(&mut self) {
        self.elements
            .iter()
            .for_each(|element| element.draw(&mut self.stdout));
        self.stdout.flush().unwrap();
    }

    pub fn add_element(&mut self, element: Box<dyn TuiElement>) {
        self.elements.push(element);
    }

    fn handle_event(&mut self, event: Event) {}
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        queue!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    }
}
