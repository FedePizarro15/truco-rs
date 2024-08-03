use crossterm::event::{read, Event, KeyCode::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use crossterm::{cursor::MoveTo, queue, style::Print};
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
        use crossterm::{cursor::MoveTo, queue, style::Print};
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
            use crossterm::{cursor::MoveTo, queue, style::Print};
            queue!($s, MoveTo($x, $y)).unwrap();
            $(queue!($s, Print($arg)).unwrap();)*
        }};
    }

#[derive(Builder)]
pub struct Tui {
    // ...
    // The terminal size
    #[builder(default = "(0, 0)", setter(into))]
    size: (u16, u16),
    // The terminal cursor position
    #[builder(default = "Cursor::default()")]
    cursor: Cursor,
    // The elements to draw
    #[builder(default = "Vec::new()")]
    elements: Vec<Box<dyn TuiElement>>,
    // The terminal stdout
    #[builder(default = "stdout()", setter(skip))]
    pub stdout: io::Stdout,
}

impl TuiBuilder {
    // Esta es la función extra que quieres ejecutar
    fn extra_function(&self, stdout: &mut Stdout) {
        enable_raw_mode().unwrap();
        queue!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
        queue!(stdout, crossterm::cursor::Hide).unwrap();
    }

    // Implementa un método build personalizado
    pub fn build_and_init(&self) -> Result<Tui, String> {
        let mut tui = self.build().map_err(|e| e.to_string())?;
        self.extra_function(&mut tui.stdout); // Llama a la función extra con el stdout de Tui
        Ok(tui)
    }
}

impl Tui {
    // Draw should not alter the state of the Tui
    pub fn draw(&mut self) {
        queue!(self.stdout, Clear(crossterm::terminal::ClearType::All)).unwrap();
        self.elements
            .iter()
            .for_each(|element| element.draw(&mut self.stdout));
        self.stdout.flush().unwrap();
    }

    pub fn add_element(&mut self, element: Box<dyn TuiElement>) {
        self.elements.push(element);
    }

    fn handle_event(&mut self, event: Event) {
        unimplemented!()
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        queue!(self.stdout, crossterm::cursor::Show).unwrap();
        queue!(self.stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

// * Iterador para los elementos de Tui

pub struct TuiElementIter<'a> {
    elements: std::slice::Iter<'a, Box<dyn TuiElement>>,
}

pub struct TuiElementIterMut<'a> {
    elements: std::slice::IterMut<'a, Box<dyn TuiElement>>,
}

impl<'a> Iterator for TuiElementIter<'a> {
    type Item = &'a Box<dyn TuiElement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next()
    }
}

impl<'a> Iterator for TuiElementIterMut<'a> {
    type Item = &'a mut Box<dyn TuiElement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.next()
    }
}

impl Tui {
    pub fn iter_elements(&self) -> TuiElementIter {
        TuiElementIter {
            elements: self.elements.iter(),
        }
    }

    pub fn iter_elements_mut(&mut self) -> TuiElementIterMut {
        TuiElementIterMut {
            elements: self.elements.iter_mut(),
        }
    }
}

// * Implementación de get_from_selection

impl Tui {}
