use std::io::Stdout;

use crossterm::{queue, style::Print};

use super::{button::TuiButton, element::TuiElement};

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
