use std::{any::Any, io::{self, stdout, Stdout, Write}};

pub trait TuiElement: TuiElementClone {
    fn draw(&self, stdout: &mut Stdout);
    fn change_position(&mut self, loc: Option<TuiElementLocation>);
    fn get_position(&self) -> (u16, u16);
    fn get_type(&self) -> TuiElementType;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

}

// This is a workaround to allow cloning Box<dyn TuiElement>
pub trait TuiElementClone {
    fn clone_box(&self) -> Box<dyn TuiElement>;
}

impl<T> TuiElementClone for T
where
    T: 'static + TuiElement + Clone,
{
    fn clone_box(&self) -> Box<dyn TuiElement> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn TuiElement> {
    fn clone(&self) -> Box<dyn TuiElement> {
        self.clone_box()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // ? Forces the compiler to check for non-exhaustive matches: match _ { ... , _ => {} }
pub enum TuiElementType {
    Button,
    Cursor,
    Input,
    Selection,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiElementLocation {
    Left,
    Center,
    Right,
    Up,
    Down,
    Absolute((u16, u16)),
}

impl TuiElementLocation {
    pub fn to_absolute(&self, term_size: (u16, u16), element_size: (u16, u16)) -> (u16, u16) {
        match self {
            TuiElementLocation::Absolute((x, y)) => (*x, *y),
            TuiElementLocation::Left => (0, term_size.1 / 2),
            TuiElementLocation::Center => (
                term_size.0 / 2 - element_size.0 / 2,
                term_size.1 / 2 - element_size.1 / 2,
            ),
            TuiElementLocation::Right => (term_size.0 - element_size.0 - 1, term_size.1 / 2),
            TuiElementLocation::Up => (term_size.0 / 2 - element_size.0 / 2, 0),
            TuiElementLocation::Down => (term_size.0 / 2 - element_size.0 / 2, term_size.1 - element_size.1),
        }
    }
}

