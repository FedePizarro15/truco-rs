use std::io::Stdout;

use crossterm::{cursor::MoveTo, queue, style::Print};
use serde::{Deserialize, Serialize};

use super::element::TuiElement;

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
    fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    fn get_type(&self) -> super::element::TuiElementType {
        super::element::TuiElementType::Input
    }
}