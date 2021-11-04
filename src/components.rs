use bracket_terminal::prelude::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Renderable {
    pub loc: Point,
    pub glyph: u8,
}

#[derive(Component)]
pub struct Player;


