use std::io::Stdout;

use crossterm::{cursor::MoveTo, queue, style::Print};
use serde::{Deserialize, Serialize};

use crate::{tui_debug, tui_print_at};

use super::element::{TuiElement, TuiElementType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuiButtonStyle {
    FullBox,
    VerticalBox,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiButton {
    x: u16,           // up-left corner of the button at x
    y: u16,           // up-left corner of the button at y
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
    fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    fn get_type(&self) -> TuiElementType {
        TuiElementType::Button
    }
    fn draw(&self, stdout: &mut Stdout) {
        let (text_x, text_y) = (
            self.size.0 / 2 + self.x - (self.text.len() as u16 / 2) + self.size.0 % 2,
            self.y + (self.size.1 / 2),
        );

        tui_debug!(
            stdout,
            format!(
                "term_size: {:?}, x: {}, y: {}, size: {:?}",
                crossterm::terminal::size(),
                self.x,
                self.y,
                self.size
            )
        );

        tui_print_at!(stdout, text_x, text_y, &self.text);

        match self.style {
            TuiButtonStyle::FullBox => {
                queue!(stdout, MoveTo(self.x, self.y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(self.x + self.size.0, self.y), Print("┐")).unwrap();
                queue!(stdout, MoveTo(self.x, self.y + self.size.1), Print("└")).unwrap();
                queue!(stdout, MoveTo(self.x + self.size.0, self.y + self.size.1), Print("┘")).unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(self.x + i, self.y), Print("─")).unwrap();
                    queue!(stdout, MoveTo(self.x + i, self.y + self.size.1), Print("─")).unwrap();
                }
                for i in 1..self.size.1 {
                    queue!(stdout, MoveTo(self.x, self.y + i), Print("│")).unwrap();
                    queue!(
                        stdout,
                        MoveTo(self.x + self.size.0, self.y + i),
                        Print("│")
                    )
                    .unwrap();
                }
            }
            TuiButtonStyle::VerticalBox => {
                queue!(stdout, MoveTo(self.x, self.y + self.size.1), Print("└")).unwrap();
                queue!(stdout, MoveTo(self.x + self.size.0, self.y + self.size.1), Print("┘")).unwrap();
                queue!(stdout, MoveTo(self.x, self.y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(self.x + self.size.0, self.y), Print("┐")).unwrap();
                
                for i in 1..self.size.1 {
                    queue!(stdout, MoveTo(self.x, self.y + i), Print("│")).unwrap();
                    queue!(
                        stdout,
                        MoveTo(self.x + self.size.0, self.y + i),
                        Print("│")
                    )
                    .unwrap();
                }
            }
            TuiButtonStyle::Underline => {
                queue!(stdout, MoveTo(self.x, self.y + self.size.1), Print("└")).unwrap();
                queue!(stdout, MoveTo(self.x + self.size.0, self.y + self.size.1), Print("┘")).unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(self.x + i, self.y + self.size.1), Print("─")).unwrap();
                }
            }
        }
    }
}

impl TuiButton {
    pub fn new(
        x: u16,
        y: u16,
        size: Option<(u16, u16)>,
        text: impl Into<String>,
        style: TuiButtonStyle,
    ) -> Self {
        let text = text.into();
        Self {
            x,
            y,
            size: {
                if let Some(size) = size {
                    size
                } else {
                    let text_len = text.len() as u16;
                    let width = text_len + 1; // Add padding of 2 characters
                    let height = 2; // Set a default height of 3
                    (width, height)
                }
            },
            text,
            style,
            selected: false,
        }
    }
}

impl Default for TuiButton {
    fn default() -> Self {
        Self::new(9, 9, None, "Text", TuiButtonStyle::Underline)
    }
}
