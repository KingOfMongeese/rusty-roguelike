use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // color pair stores fg and bg, from bracket
    pub glyph: FontCharType, // bracket, store char or glyph
}
