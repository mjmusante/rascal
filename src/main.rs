use bracket_terminal::prelude::*;

mod components;
mod game;
mod gui;
mod interface;
mod items;
mod map;

use interface::Interface;

bracket_terminal::embedded_resource!(PETSCII, "resources/petchars.png");

pub fn main() -> BError {
    bracket_terminal::link_resource!(PETSCII, "resources/petchars.png");

    let context = BTermBuilder::new()
        .with_simple_console(40, 25, "petchars.png")
        .with_title("Rascals")
        .with_tile_dimensions(16, 16)
        .with_font("petchars.png", 8, 8)
        .build()?;

    let engine = Interface::new();

    main_loop(context, engine)
}
