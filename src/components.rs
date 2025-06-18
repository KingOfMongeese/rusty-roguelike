pub use crate::prelude::*; // TODO: find out why pub use here

// ########################################
// Render
// ########################################
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // color pair stores fg and bg, from bracket
    pub glyph: FontCharType, // bracket, store char or glyph
}

// ########################################
// Tags
// ########################################
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

// ########################################
// Messages
// ########################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub defender: Entity,
}

// ########################################
// Combat
// ########################################

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

// ########################################
// Properties
// ########################################

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
