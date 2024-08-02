use std::io::{self, stdout, Stdout, Write};
pub trait TuiElement {
    fn draw(&self, stdout: &mut Stdout);
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>);
}