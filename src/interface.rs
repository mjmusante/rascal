use bracket_terminal::prelude::*;

use crate::gui::*;
use crate::game::*;

pub struct Interface {
    gui: Gui,
    game: Game,
}

impl GameState for Interface {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.gui.draw(ctx);
        self.game.run(&mut self.gui);
    }
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            gui: Gui::new(),
            game: Game::new(),
        }
    }
}
