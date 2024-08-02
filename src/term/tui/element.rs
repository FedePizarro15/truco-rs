use std::io::{self, stdout, Stdout, Write};

pub trait TuiElement: TuiElementClone {
    fn draw(&self, stdout: &mut Stdout);
    fn change_position(&mut self, x: Option<u16>, y: Option<u16>);
    fn get_position(&self) -> (u16, u16);
    fn get_type(&self) -> TuiElementType;
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