use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, stdout, Stdout, Write};

macro_rules! tui_debug {
    ($($arg:expr),*) => {
        let term_size = crossterm::terminal::size().unwrap();
        let (x, y) = term_size;
        queue!(stdout(), MoveTo(0, y - 1), Print("DEBUG: ")).unwrap();
        $(queue!(stdout(), Print($arg)).unwrap();)*
        stdout().flush().unwrap();
    };
}

macro_rules! tui_print_at {
    ($s:expr, $x:expr, $y:expr, $($arg:expr),*) => {
        queue!($s, MoveTo($x, $y)).unwrap();
        $(queue!($s, Print($arg)).unwrap();)*
    };
}

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

pub trait TuiElement {
    fn draw(&self, stdout: &mut Stdout);
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuiButtonStyle {
    FullBox,
    VerticalBox,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiButton {
    x: u16,           // middle of the button at x
    y: u16,           // middle of the button at y
    size: (u16, u16), // it will be used to calculate the middle of the button
    text: String,     // the text to display
    style: TuiButtonStyle,
    selected: bool,
}


impl TuiElement for TuiButton {
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    fn draw(&self, stdout: &mut Stdout) {
        let x = self.x - self.size.0 / 2;
        let y = self.y - self.size.1 / 2;
        let (text_x, text_y) = (x - self.text.len().try_into().unwrap_or(0) / 2, y);

        tui_debug!(format!(
            "term_size: {:?}, x: {}, y: {}, size: {:?}",
            crossterm::terminal::size(),
            x,
            y,
            self.size
        ));

        tui_print_at!(
            stdout,
            text_x,
            text_y,
            &self.text
        );

        match self.style {
            TuiButtonStyle::FullBox => {
                queue!(stdout, MoveTo(x, y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y), Print("┐")).unwrap();
                queue!(stdout, MoveTo(x, y + self.size.1), Print("└")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y + self.size.1), Print("┘")).unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(x + i, y), Print("─")).unwrap();
                    queue!(stdout, MoveTo(x + i, y + self.size.1), Print("─")).unwrap();
                }
                for i in 1..self.size.1 {
                    queue!(stdout, MoveTo(x, y + i), Print("│")).unwrap();
                    queue!(stdout, MoveTo(x + self.size.0, y + i), Print("│")).unwrap();
                }
            }
            TuiButtonStyle::VerticalBox => {
                queue!(stdout, MoveTo(x, y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(x, y + self.size.1), Print("└")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y), Print("┐")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y + self.size.1), Print("┘")).unwrap();
                for i in 1..self.size.1 {
                    queue!(stdout, MoveTo(x, y + i), Print("│")).unwrap();
                    queue!(stdout, MoveTo(x + self.size.0, y + i), Print("│")).unwrap();
                }
            }
            TuiButtonStyle::Underline => {
                queue!(stdout, MoveTo(text_x - 1, self.y), Print("└")).unwrap();
                queue!(stdout, MoveTo(text_x + self.size.0 - 1, self.y), Print("┘")).unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(text_x + i - 1, y + 1), Print("─")).unwrap();
                }
            }
        }
    }
}

impl TuiButton {
    fn new(x: u16, y: u16, size: (u16, u16), text: String, style: TuiButtonStyle) -> Self {
        Self {
            x,
            y,
            size,
            text,
            style,
            selected: false,
        }
    }
}

impl Default for TuiButton {
    fn default() -> Self {
        Self {
            x: 18,
            y: 9,
            size: (5, 3), //
            //       Text
            //     └------┘
            text: String::from("Text"),
            style: TuiButtonStyle::Underline,
            selected: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiText {
    x: u16,
    y: u16,
    text: String,
}

impl TuiElement for TuiText {
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    fn draw(&self, stdout: &mut Stdout) {
        queue!(stdout, MoveTo(self.x, self.y), Print(&self.text)).unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiInput {
    x: u16,
    y: u16,
    text: String,
}

impl TuiElement for TuiInput {
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    fn draw(&self, stdout: &mut Stdout) {
        queue!(stdout, MoveTo(self.x, self.y), Print(&self.text)).unwrap();
    }
}

pub struct TuiSelection {
    x: u16,
    y: u16,
    options: Vec<TuiButton>,
    selected: usize,
}

impl TuiElement for TuiSelection {
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>) {
        if let Some(x) = x {
            self.x = x;
        }
        if let Some(y) = y {
            self.y = y;
        }
    }
    fn draw(&self, stdout: &mut Stdout) {
        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected {
                queue!(stdout, Print("> ")).unwrap();
            } else {
                queue!(stdout, Print("  ")).unwrap();
            }
            option.draw(stdout);
        }
    }
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
            cursor: Cursor { x: 0, y: 0 },
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
