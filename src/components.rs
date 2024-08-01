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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
    pub name: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Item;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AmuletOfYala;