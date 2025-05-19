pub use crate::prelude::*; // TODO: find out why pub use here

/// contains render data
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // color pair stores fg and bg, from bracket
    pub glyph: FontCharType, // bracket, store char or glyph
}

/// tags Player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// tags Enemy
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
