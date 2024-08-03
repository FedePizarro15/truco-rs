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
    fn change_position(&mut self, loc: Option<super::element::TuiElementLocation>) {
        if let Some(loc) = loc {
            let (x, y) = loc.to_absolute(crossterm::terminal::size().unwrap(), (self.text.len() as u16, 1));
            self.x = x;
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}