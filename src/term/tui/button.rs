use std::io::Stdout;

use crossterm::{cursor::MoveTo, queue, style::Print};
use serde::{Deserialize, Serialize};

use crate::{tui_debug, tui_print_at};

use super::element::TuiElement;

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

        tui_debug!(
            stdout,
            format!(
                "term_size: {:?}, x: {}, y: {}, size: {:?}",
                crossterm::terminal::size(),
                x,
                y,
                self.size
            )
        );

        tui_print_at!(stdout, text_x, text_y, &self.text);

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
