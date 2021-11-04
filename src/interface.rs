use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::components::*;

use crate::game::*;
use crate::gui::*;

pub struct Interface {
    gui: Gui,
    game: Game,
    ecs: World,
}

impl GameState for Interface {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            self.game.handle_key(&mut self.ecs, key);
            self.run_systems();
        }

        ctx.cls();
        self.gui.draw(ctx);

        {
            let renderables = self.ecs.read_storage::<Renderable>();
            let offset = self.game.map_offset();

            for r in (&renderables).join() {
                ctx.set(r.loc.x - offset.0, r.loc.y - offset.1,
                        RGB::named(GREEN), RGB::named(BLACK), r.glyph);
            }
        }


        self.game.run(&mut self.gui, &mut self.ecs);
    }
}

impl Interface {
    pub fn new() -> Interface {
        let mut ecs = World::new();
        let game = Game::new();

        ecs.register::<Player>();
        ecs.register::<Renderable>();

        let loc = game.initial_loc();

        let player = ecs.create_entity()
            .with(Player{})
            .with(Renderable{loc, glyph: 0})
            .build();
        ecs.insert(player);

        Interface {
            gui: Gui::new(),
            game, ecs,
        }
    }

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}
