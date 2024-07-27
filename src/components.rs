pub use crate::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Player;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Enemy;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MovingRandomly;