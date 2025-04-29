pub use crate::prelude::*; // TODO: find out why pub use here

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair, // color pair stores fg and bg, from bracket
    pub glyph: FontCharType, // bracket, store char or glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;