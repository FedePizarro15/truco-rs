use std::io::Stdout;

use crossterm::{queue, style::Print};

use super::{button::TuiButton, element::TuiElement};
#[derive(Debug, Clone)]
pub struct TuiSelection {
    x: u16,
    y: u16,
    options: Vec<TuiButton>,
    selected: usize,
}

impl TuiElement for TuiSelection {
    fn change_position(&mut self, loc: Option<super::element::TuiElementLocation>) {
        unimplemented!()
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
    fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    fn get_type(&self) -> super::element::TuiElementType {
        super::element::TuiElementType::Selection
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
