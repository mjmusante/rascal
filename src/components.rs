use bracket_terminal::prelude::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
}

#[derive(PartialEq, Component)]
pub struct Position {
    pub loc: Point,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct WantsToPickUp {
    pub collected_by: Entity,
    pub entity: Entity,
}

pub fn register_components(ecs: &mut World) {
    ecs.register::<Player>();
    ecs.register::<Renderable>();
    ecs.register::<Position>();
    ecs.register::<Item>();
    ecs.register::<Name>();
    ecs.register::<InBackpack>();
    ecs.register::<WantsToPickUp>();
}
