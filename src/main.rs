use bracket_terminal::prelude::*;

mod interface;
mod gui;
mod game;

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

        let gui = Interface::new();
        
        main_loop(context, gui)
}
