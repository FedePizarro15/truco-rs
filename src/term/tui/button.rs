use std::io::Stdout;

use crossterm::{cursor::MoveTo, queue, style::Print};
use serde::{Deserialize, Serialize};

use crate::{term::tui::text, tui_debug, tui_print_at};

use super::{
    element::{TuiElement, TuiElementLocation, TuiElementType},
    Tui,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuiButtonStyle {
    FullBox,
    VerticalBox,
    Underline,
}

#[derive(Debug, Clone)]
pub struct TuiButton {
    x: u16,           // up-left corner of the button at x
    y: u16,           // up-left corner of the button at y
    size: (u16, u16), // it will be used to calculate the middle of the button
    text: String,     // the text to display
    style: TuiButtonStyle,
    selected: bool,
}

impl TuiElement for TuiButton {
    fn change_position(&mut self, loc: Option<TuiElementLocation>) {
        if let Some(loc) = loc {
            let (x, y) = loc.to_absolute(crossterm::terminal::size().unwrap(), self.size);
            self.x = x;
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
        let text_len = self.text.len() as u16;
        let mut x = self.x;
        let mut y = self.y;

        if self.x + self.size.0 >= crossterm::terminal::size().unwrap().0 {
            x = crossterm::terminal::size().unwrap().0 - self.size.0 - 1;
        }

        let (text_x, text_y) = (
            self.size.0 / 2 + x - (text_len / 2) + (self.size.0 % 2),
            y + (self.size.1 / 2),
        );
        // tui_debug!(
        //     stdout,
        //     format!(
        //         "term_size: {:?}, x: {:?}, y: {:?}, size: {:?}, text_len: {}",
        //         crossterm::terminal::size(),
        //         x,
        //         y,
        //         self.size,
        //         self.text.len()
        //     )
        // );
        tui_print_at!(stdout, text_x, text_y, &self.text);

        match self.style {
            TuiButtonStyle::FullBox => {
                queue!(stdout, MoveTo(x, y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y), Print("┐")).unwrap();
                queue!(
                    stdout,
                    MoveTo(x, y + self.size.1 - self.size.1 % 2),
                    Print("└")
                )
                .unwrap();
                queue!(
                    stdout,
                    MoveTo(x + self.size.0, y + self.size.1 - self.size.1 % 2),
                    Print("┘")
                )
                .unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(x + i, y), Print("─")).unwrap();
                    queue!(
                        stdout,
                        MoveTo(
                            x + i,
                            y + self.size.1.saturating_sub(self.size.1 % 2)
                        ),
                        Print("─")
                    )
                    .unwrap();
                }
                for i in 1..(self.size.1 - self.size.1 % 2) {
                    queue!(stdout, MoveTo(x, y + i), Print("│")).unwrap();
                    queue!(stdout, MoveTo(x + self.size.0, y + i), Print("│")).unwrap();
                }
            }
            TuiButtonStyle::VerticalBox => {
                queue!(stdout, MoveTo(x, y + self.size.1), Print("└")).unwrap();
                queue!(
                    stdout,
                    MoveTo(x + self.size.0, y + self.size.1),
                    Print("┘")
                )
                .unwrap();
                queue!(stdout, MoveTo(x, y), Print("┌")).unwrap();
                queue!(stdout, MoveTo(x + self.size.0, y), Print("┐")).unwrap();

                for i in 1..self.size.1 {
                    queue!(stdout, MoveTo(x, y + i), Print("│")).unwrap();
                    queue!(stdout, MoveTo(x + self.size.0, y + i), Print("│")).unwrap();
                }
            }
            TuiButtonStyle::Underline => {
                queue!(stdout, MoveTo(x, y + self.size.1), Print("└")).unwrap();
                queue!(
                    stdout,
                    MoveTo(x + self.size.0, y + self.size.1),
                    Print("┘")
                )
                .unwrap();
                for i in 1..self.size.0 {
                    queue!(stdout, MoveTo(x + i, y + self.size.1), Print("─")).unwrap();
                }
            }
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl TuiButton {
    pub fn new(
        loc: TuiElementLocation,
        size: Option<(u16, u16)>,
        text: impl Into<String>,
        style: TuiButtonStyle,
    ) -> Self {
        let text = text.into();
        let (x, y) = loc.to_absolute(
            crossterm::terminal::size().unwrap(),
            size.unwrap_or((text.len() as u16 + 1, 3)),
        );
        Self {
            x,
            y,
            size: {
                if let Some(size) = size {
                    size
                } else {
                    let text_len = text.len() as u16;
                    let width = text_len + 1; // Add padding of 2 characters
                    let height = match style {
                        TuiButtonStyle::FullBox => 3,
                        TuiButtonStyle::VerticalBox => 2,
                        TuiButtonStyle::Underline => 2,
                    };
                    (width, height)
                }
            },
            text,
            style,
            selected: false,
        }
    }
    pub fn mutate_text(&mut self, new_text: impl Into<String>) {
        self.text = new_text.into();
        let text_len = self.text.len() as u16;
        let width = text_len + 1; // Add padding of 2 characters
        let height = match self.style {
            TuiButtonStyle::FullBox => 3,
            TuiButtonStyle::VerticalBox => 2,
            TuiButtonStyle::Underline => 2,
        };
        self.size = (width, height);
    }
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn unselect(&mut self) {
        self.selected = false;
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl Default for TuiButton {
    fn default() -> Self {
        Self::new(TuiElementLocation::Absolute((0,0)), None, "Text", TuiButtonStyle::Underline)
    }
}
